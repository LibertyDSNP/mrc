use super::mock::*;
use crate::{stateful_child_tree::StatefulChildTree, types::*, Config, Error};
use codec::{Decode, Encode, MaxEncodedLen};
use common_primitives::schema::{ModelType, PayloadLocation, SchemaId};
use frame_support::{assert_err, assert_ok, BoundedVec};
#[allow(unused_imports)]
use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};
use scale_info::TypeInfo;
use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};

type TestPageSize = <Test as Config>::MaxItemizedPageSizeBytes;
type TestPage = Page<TestPageSize>;

fn generate_payload_bytes(id: Option<u8>) -> BoundedVec<u8, TestPageSize> {
	let value = id.unwrap_or(1);
	format!("{{'type':{value}, 'description':'another test description {value}'}}")
		.as_bytes()
		.to_vec()
		.try_into()
		.unwrap()
}

fn create_itemized_page_from(payloads: &[BoundedVec<u8, TestPageSize>]) -> TestPage {
	let mut buffer: Vec<u8> = vec![];
	for p in payloads {
		buffer.extend_from_slice(&ItemHeader { payload_len: p.len() as u16 }.encode()[..]);
		buffer.extend_from_slice(p.as_slice());
	}
	TestPage::try_from(buffer).unwrap()
}

#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
/// A structure defining a Schema
struct TestStruct {
	pub model_type: ModelType,
	pub payload_location: PayloadLocation,
	pub number: u64,
}

#[test]
fn upsert_page_too_large_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 5;
		let msa_id = 1;
		let schema_id = 1;
		let page_id = 0;
		let payload =
			vec![
				1;
				TryInto::<usize>::try_into(<Test as Config>::MaxPaginatedPageSizeBytes::get())
					.unwrap() + 1
			];

		assert_err!(
			StatefulStoragePallet::upsert_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				payload
			),
			Error::<Test>::PageExceedsMaxPageSizeBytes
		)
	})
}

#[test]
fn upsert_page_id_out_of_bounds_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 5;
		let msa_id = 1;
		let schema_id = 1;
		let page_id = <Test as Config>::MaxPaginatedPageId::get() + 1;
		let payload = vec![1; 1];

		assert_err!(
			StatefulStoragePallet::upsert_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				payload
			),
			Error::<Test>::PageIdExceedsMaxAllowed
		)
	})
}

#[test]
fn upsert_page_with_invalid_msa_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1000; // hard-coded in mocks to return None for MSA
		let msa_id = 1;
		let schema_id = 1;
		let page_id = 1;
		let payload = vec![1; 1];

		assert_err!(
			StatefulStoragePallet::upsert_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				payload
			),
			Error::<Test>::InvalidMessageSourceAccount
		)
	})
}

#[test]
fn upsert_page_with_invalid_schema_id_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 1;
		let schema_id = INVALID_SCHEMA_ID;
		let page_id = 1;
		let payload = vec![1; 1];

		assert_err!(
			StatefulStoragePallet::upsert_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				payload
			),
			Error::<Test>::InvalidSchemaId
		)
	})
}

#[test]
fn upsert_page_with_invalid_schema_payload_location_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 1;
		let schema_id = ITEMIZED_SCHEMA;
		let page_id = 1;
		let payload = vec![1; 1];

		assert_err!(
			StatefulStoragePallet::upsert_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				payload
			),
			Error::<Test>::SchemaPayloadLocationMismatch
		)
	})
}

#[test]
fn upsert_page_with_no_delegation_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 1;
		let schema_id = UNDELEGATED_PAGINATED_SCHEMA;
		let page_id = 1;
		let payload = vec![1; 1];

		assert_err!(
			StatefulStoragePallet::upsert_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				payload
			),
			Error::<Test>::UnAuthorizedDelegate
		)
	})
}

#[test]
fn upsert_new_page_succeeds() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 1;
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 1;
		let payload = generate_payload_bytes(Some(100));
		let page: TestPage = payload.clone().into();

		assert_ok!(StatefulStoragePallet::upsert_page(
			RuntimeOrigin::signed(caller_1),
			msa_id,
			schema_id,
			page_id,
			payload.into()
		));

		let keys = (schema_id, page_id);
		let new_page: TestPage = <StatefulChildTree>::try_read(&msa_id, &keys).unwrap().unwrap();
		assert_eq!(page, new_page);
	})
}

#[test]
fn upsert_existing_page_modifies_page() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 2;
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 1;
		let old_content = generate_payload_bytes(Some(200));
		let new_content = generate_payload_bytes(Some(201));
		let old_page: TestPage = old_content.clone().into();

		let keys = (schema_id, page_id);
		<StatefulChildTree>::write(&msa_id, &keys, old_page);
		let old_page: TestPage = <StatefulChildTree>::try_read(&msa_id, &keys).unwrap().unwrap();

		assert_eq!(old_content, old_page.data);
		assert_ok!(StatefulStoragePallet::upsert_page(
			RuntimeOrigin::signed(caller_1),
			msa_id,
			schema_id,
			page_id,
			new_content.clone().into()
		));

		let new_page: TestPage = <StatefulChildTree>::try_read(&msa_id, &keys).unwrap().unwrap();
		assert_eq!(new_content, new_page.data);
	})
}

#[test]
fn remove_page_id_out_of_bounds_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 2;
		let schema_id = PAGINATED_SCHEMA;
		let page_id = <Test as Config>::MaxPaginatedPageId::get() + 1;

		assert_err!(
			StatefulStoragePallet::remove_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
			),
			Error::<Test>::PageIdExceedsMaxAllowed
		);
	})
}

#[test]
fn remove_page_with_invalid_msa_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1000; // hard-coded in mocks to return None for MSA
		let msa_id = 1;
		let schema_id = 1;
		let page_id = 1;

		assert_err!(
			StatefulStoragePallet::remove_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
			),
			Error::<Test>::InvalidMessageSourceAccount
		)
	})
}

#[test]
fn remove_page_with_invalid_schema_id_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 1;
		let schema_id = INVALID_SCHEMA_ID;
		let page_id = 1;

		assert_err!(
			StatefulStoragePallet::remove_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
			),
			Error::<Test>::InvalidSchemaId
		)
	})
}

#[test]
fn remove_page_with_invalid_schema_payload_location_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 1;
		let schema_id = ITEMIZED_SCHEMA;
		let page_id = 1;

		assert_err!(
			StatefulStoragePallet::remove_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
			),
			Error::<Test>::SchemaPayloadLocationMismatch
		)
	})
}

#[test]
fn remove_page_with_no_delegation_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 1;
		let schema_id = UNDELEGATED_PAGINATED_SCHEMA;
		let page_id = 1;

		assert_err!(
			StatefulStoragePallet::remove_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
			),
			Error::<Test>::UnAuthorizedDelegate
		)
	})
}

#[test]
fn remove_nonexistent_page_succeeds_noop() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 1;
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 10;

		// TODO: Get list of existing pages to verify the page doesn't already exist
		// PROBLEM: Child Trie iterator doesn't appear to yield keys
		// let key = schema_id.encode().to_vec();
		// let i = StatefulChildTree::prefix_iterator::<Vec<u8>>(&msa_id, &[key]);
		// let existing_node = i.filter(|k,v| {})

		assert_ok!(StatefulStoragePallet::remove_page(
			RuntimeOrigin::signed(caller_1),
			msa_id,
			schema_id,
			page_id
		));
	})
}

#[test]
fn remove_existing_page_succeeds() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = 1;
		let msa_id = 1;
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 11;
		let payload = generate_payload_bytes(None);

		assert_ok!(StatefulStoragePallet::upsert_page(
			RuntimeOrigin::signed(caller_1),
			msa_id,
			schema_id,
			page_id,
			payload.into()
		));

		assert_ok!(StatefulStoragePallet::remove_page(
			RuntimeOrigin::signed(caller_1),
			msa_id,
			schema_id,
			page_id
		));

		let keys = (schema_id, page_id);
		let page: Option<TestPage> = <StatefulChildTree>::try_read(&msa_id, &keys).unwrap();
		assert_eq!(page, None);
	})
}

#[test]
fn parsing_a_well_formed_item_page_should_work() {
	// arrange
	let payloads = vec![generate_payload_bytes(Some(1)), generate_payload_bytes(Some(2))];
	let page = create_itemized_page_from(&payloads);

	// act
	let parsed = page.parse_as_itemized();

	// assert
	assert_ok!(&parsed);
	assert_eq!(
		parsed.as_ref().unwrap().page_size,
		payloads.len() * ItemHeader::max_encoded_len() +
			payloads.iter().map(|p| p.len()).sum::<usize>()
	);

	let items = parsed.unwrap().items;
	for index in 0..payloads.len() {
		assert_eq!(
			items.get(&(index as u16)).unwrap()[ItemHeader::max_encoded_len()..],
			payloads[index][..]
		);
	}
}

#[test]
fn parsing_item_with_wrong_payload_size_should_return_parsing_error() {
	// arrange
	let payload = generate_payload_bytes(Some(1));
	let mut buffer: Vec<u8> = vec![];
	buffer.extend_from_slice(&ItemHeader { payload_len: (payload.len() + 1) as u16 }.encode()[..]);
	buffer.extend_from_slice(&payload);
	let page: TestPage = Page::try_from(buffer).unwrap();

	// act
	let parsed = page.parse_as_itemized();

	// assert
	assert_eq!(parsed, Err(PageError::ErrorParsing("wrong payload size")));
}

#[test]
fn applying_remove_action_with_existing_index_should_remove_item() {
	// arrange
	let payloads = vec![generate_payload_bytes(Some(2)), generate_payload_bytes(Some(4))];
	let page = create_itemized_page_from(payloads.as_slice());
	let expecting_page = create_itemized_page_from(&payloads[1..]);
	let actions = vec![ItemAction::Remove { index: 0 }];

	// act
	let result = page.apply_item_actions(&actions);

	// assert
	assert_ok!(&result);
	let updated = result.unwrap();
	assert_eq!(expecting_page.data, updated.data);
}

#[test]
fn applying_add_action_should_add_item_to_the_end_of_the_page() {
	// arrange
	let payload1 = vec![generate_payload_bytes(Some(2))];
	let page = create_itemized_page_from(payload1.as_slice());
	let payload2 = vec![generate_payload_bytes(Some(4))];
	let expecting_page =
		create_itemized_page_from(&vec![payload1[0].clone(), payload2[0].clone()][..]);
	let actions = vec![ItemAction::Add { data: payload2[0].clone().into() }];

	// act
	let result = page.apply_item_actions(&actions[..]);

	// assert
	assert_ok!(&result);
	let updated = result.unwrap();
	assert_eq!(expecting_page.data, updated.data);
}

#[test]
fn applying_remove_action_with_non_existing_index_should_fail() {
	// arrange
	let payloads = vec![generate_payload_bytes(Some(2)), generate_payload_bytes(Some(4))];
	let page = create_itemized_page_from(payloads.as_slice());
	let actions = vec![ItemAction::Remove { index: 2 }];

	// act
	let result = page.apply_item_actions(&actions[..]);

	// assert
	assert_eq!(result.is_err(), true);
}

#[test]
fn applying_add_action_with_full_page_should_fail() {
	// arrange
	let new_payload = generate_payload_bytes(Some(2));
	let page_len = TestPageSize::get() as usize - ItemHeader::max_encoded_len() - new_payload.len();
	let existing_data = vec![1u8; page_len];
	let existing_payload: BoundedVec<u8, TestPageSize> =
		BoundedVec::try_from(existing_data).unwrap();
	let page = create_itemized_page_from(&[existing_payload]);
	let actions = vec![ItemAction::Add { data: new_payload.clone().into() }];

	// act
	let result = page.apply_item_actions(&actions[..]);

	// assert
	assert_eq!(result, Err(PageError::PageSizeOverflow));
}

#[test]
fn is_empty_false_for_non_empty_page() {
	let page: TestPage = vec![1].try_into().unwrap();

	assert_eq!(page.is_empty(), false);
}

#[test]
fn is_empty_true_for_empty_page() {
	let page: TestPage = Vec::<u8>::new().try_into().unwrap();

	assert_eq!(page.is_empty(), true);
}

#[test]
fn child_tree_write_read() {
	new_test_ext().execute_with(|| {
		// arrange
		let msa_id = 1;
		let schema_id: SchemaId = 2;
		let page_id: u8 = 3;
		let keys = &(schema_id, page_id);
		let val = TestStruct {
			model_type: ModelType::AvroBinary,
			payload_location: PayloadLocation::OnChain,
			number: 8276387272,
		};

		// act
		<StatefulChildTree>::write(&msa_id, keys, &val);

		// assert
		let read = <StatefulChildTree>::try_read(&msa_id, keys).unwrap();
		assert_eq!(Some(val), read);
	});
}

#[test]
fn child_tree_iterator() {
	new_test_ext().execute_with(|| {
		// arrange
		let msa_id = 1;
		let mut arr: Vec<TestStruct> = Vec::new();
		let prefix_1 = "part_1".to_string();
		let prefix_2a = "part_2a".to_string();
		let prefix_2b = "part_2b".to_string();

		for i in 1u64..=10u64 {
			let k = (
				prefix_1.clone(),
				match i % 2 {
					0 => prefix_2a.clone(),
					_ => prefix_2b.clone(),
				},
				i.clone(),
			);
			let s = TestStruct {
				model_type: ModelType::AvroBinary,
				payload_location: PayloadLocation::OnChain,
				number: i,
			};
			arr.push(s.clone());
			<StatefulChildTree>::write(&msa_id, &k, s);
		}

		// Try 1-level prefix
		let prefix_key = (prefix_1.clone(),);
		// let mut v: Vec<((String, String, u64), TestStruct)> = Vec::new();
		let nodes = <StatefulChildTree>::prefix_iterator::<_, TestStruct, (String, u64)>(
			&msa_id,
			&prefix_key.clone(),
		);
		let r0: BTreeSet<u64> = nodes.map(|(_, v)| v.number).collect();

		// Should return all items
		assert_eq!(r0, arr.iter().map(|s| s.number).collect());

		// Try 2-level prefix
		let prefix_key = (prefix_1.clone(), prefix_2a.clone());
		let nodes2 =
			<StatefulChildTree>::prefix_iterator::<_, TestStruct, (u64,)>(&msa_id, &prefix_key);
		let r1: BTreeSet<u64> = nodes2.map(|(_, v)| v.number).collect();

		// Should return only even-numbered items
		assert_eq!(r1, arr.iter().filter(|s| s.number % 2 == 0).map(|s| s.number).collect());
	});
}
