use crate::{
	stateful_child_tree::StatefulChildTree,
	tests::{
		mock::*,
		test_common::{constants::*, test_constants::*},
	},
	types::*,
	Config, Error, Event as StatefulEvent,
};
use codec::Encode;
use common_primitives::{stateful_storage::PageHash, utils::wrap_binary_data};
use frame_support::{assert_err, assert_ok, assert_storage_noop};
#[allow(unused_imports)]
use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};
use sp_core::{sr25519, Pair};
use sp_runtime::MultiSignature;

#[test]
fn delete_page_id_out_of_bounds_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let msa_id = 1;
		let caller_1 = test_public(msa_id);
		let schema_id = PAGINATED_SCHEMA;
		let page_id = <Test as Config>::MaxPaginatedPageId::get() + 1;

		assert_err!(
			StatefulStoragePallet::delete_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				NONEXISTENT_PAGE_HASH,
			),
			Error::<Test>::PageIdExceedsMaxAllowed
		);
	})
}

#[test]
fn delete_page_with_invalid_msa_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = test_public(INVALID_MSA_ID); // hard-coded in mocks to return None for MSA
		let msa_id = 1;
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 1;

		assert_err!(
			StatefulStoragePallet::delete_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				NONEXISTENT_PAGE_HASH,
			),
			Error::<Test>::InvalidMessageSourceAccount
		)
	})
}

#[test]
fn delete_page_with_invalid_schema_id_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let msa_id = 1;
		let caller_1 = test_public(1);
		let schema_id = INVALID_SCHEMA_ID;
		let page_id = 1;

		assert_err!(
			StatefulStoragePallet::delete_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				NONEXISTENT_PAGE_HASH,
			),
			Error::<Test>::InvalidSchemaId
		)
	})
}

#[test]
fn delete_page_with_invalid_schema_payload_location_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let msa_id = 1;
		let caller_1 = test_public(msa_id);
		let schema_id = ITEMIZED_SCHEMA;
		let page_id = 1;

		assert_err!(
			StatefulStoragePallet::delete_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				NONEXISTENT_PAGE_HASH,
			),
			Error::<Test>::SchemaPayloadLocationMismatch
		)
	})
}

#[test]
fn delete_page_with_no_delegation_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let msa_id = 1;
		let caller_1 = test_public(2);
		let schema_id = UNDELEGATED_PAGINATED_SCHEMA;
		let page_id = 1;

		assert_err!(
			StatefulStoragePallet::delete_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				NONEXISTENT_PAGE_HASH,
			),
			Error::<Test>::UnauthorizedDelegate
		)
	})
}

#[test]
fn delete_nonexistent_page_succeeds_noop() {
	new_test_ext().execute_with(|| {
		// setup
		let msa_id = 1;
		let caller_1 = test_public(msa_id);
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 10;

		assert_ok!(StatefulStoragePallet::delete_page(
			RuntimeOrigin::signed(caller_1),
			msa_id,
			schema_id,
			page_id,
			NONEXISTENT_PAGE_HASH,
		));
	})
}

#[test]
fn delete_existing_page_with_bad_hash_errors() {
	new_test_ext().execute_with(|| {
		// setup
		let msa_id = 1;
		let caller_1 = test_public(msa_id);
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 11;
		let page: PaginatedPage<Test> = generate_page(None, None);

		let keys = (schema_id, page_id);
		<StatefulChildTree>::write(
			&msa_id,
			PALLET_STORAGE_PREFIX,
			PAGINATED_STORAGE_PREFIX,
			&keys,
			&page,
		);

		assert_err!(
			StatefulStoragePallet::delete_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				0u32,
			),
			Error::<Test>::StalePageState
		);
	})
}

#[test]
fn delete_existing_page_succeeds() {
	new_test_ext().execute_with(|| {
		// setup
		let msa_id = 1;
		let caller_1 = test_public(msa_id);
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 11;
		let page: PaginatedPage<Test> = generate_page(None, None);
		let page_hash = page.get_hash();
		let keys = (schema_id, page_id);

		<StatefulChildTree>::write(
			&msa_id,
			PALLET_STORAGE_PREFIX,
			PAGINATED_STORAGE_PREFIX,
			&keys,
			&page,
		);

		assert_ok!(StatefulStoragePallet::delete_page(
			RuntimeOrigin::signed(caller_1),
			msa_id,
			schema_id,
			page_id,
			page_hash
		));

		let page: Option<PaginatedPage<Test>> = <StatefulChildTree>::try_read(
			&msa_id,
			PALLET_STORAGE_PREFIX,
			PAGINATED_STORAGE_PREFIX,
			&keys,
		)
		.unwrap();
		assert_eq!(page, None);
	})
}

#[test]
fn delete_page_with_signature_having_page_id_out_of_bounds_should_fail() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let (msa_id, pair) = get_signature_account();
		let delegator_key = pair.public();
		let schema_id = UNDELEGATED_PAGINATED_SCHEMA;
		let page_id = (<Test as Config>::MaxPaginatedPageId::get() + 1).into();
		let payload = PaginatedDeleteSignaturePayload {
			target_hash: PageHash::default(),
			msa_id,
			expiration: 10,
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = pair.sign(&encode_data_new_key_data).into();

		// act
		assert_err!(
			StatefulStoragePallet::delete_page_with_signature(
				RuntimeOrigin::signed(caller_1),
				delegator_key.into(),
				owner_signature,
				payload
			),
			Error::<Test>::PageIdExceedsMaxAllowed
		)
	})
}

#[test]
fn delete_page_with_signature_having_expired_payload_should_fail() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let (msa_id, pair) = get_signature_account();
		let delegator_key = pair.public();
		let schema_id = UNDELEGATED_PAGINATED_SCHEMA;
		let page_id = 1;
		let block_number = 10;
		let payload = PaginatedDeleteSignaturePayload {
			target_hash: PageHash::default(),
			msa_id,
			expiration: 10,
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = pair.sign(&encode_data_new_key_data).into();

		// act
		System::set_block_number(block_number);
		assert_err!(
			StatefulStoragePallet::delete_page_with_signature(
				RuntimeOrigin::signed(caller_1),
				delegator_key.into(),
				owner_signature,
				payload
			),
			Error::<Test>::ProofHasExpired
		)
	})
}

#[test]
fn delete_page_with_signature_having_out_of_window_payload_should_fail() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let (msa_id, pair) = get_signature_account();
		let delegator_key = pair.public();
		let schema_id = UNDELEGATED_PAGINATED_SCHEMA;
		let page_id = 1;
		let payload = PaginatedDeleteSignaturePayload {
			target_hash: PageHash::default(),
			msa_id,
			expiration: (<Test as Config>::MortalityWindowSize::get() + 1).into(),
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = pair.sign(&encode_data_new_key_data).into();

		// act
		assert_err!(
			StatefulStoragePallet::delete_page_with_signature(
				RuntimeOrigin::signed(caller_1),
				delegator_key.into(),
				owner_signature,
				payload
			),
			Error::<Test>::ProofNotYetValid
		)
	})
}

#[test]
fn delete_page_with_signature_having_wrong_signature_should_fail() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let (msa_id, pair) = get_signature_account();
		let delegator_key = pair.public();
		let (signature_key, _) = sr25519::Pair::generate();
		let schema_id = UNDELEGATED_PAGINATED_SCHEMA;
		let page_id = 1;
		let payload = PaginatedDeleteSignaturePayload {
			target_hash: PageHash::default(),
			msa_id,
			expiration: 10,
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = signature_key.sign(&encode_data_new_key_data).into();

		// act
		assert_err!(
			StatefulStoragePallet::delete_page_with_signature(
				RuntimeOrigin::signed(caller_1),
				delegator_key.into(),
				owner_signature,
				payload
			),
			Error::<Test>::InvalidSignature
		)
	})
}

#[test]
fn delete_page_with_signature_having_non_existing_msa_should_fail() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let pair = get_invalid_msa_signature_account(); // hardcoded key that returns None Msa
		let delegator_key = pair.public();
		let msa_id = 1;
		let schema_id = UNDELEGATED_PAGINATED_SCHEMA;
		let page_id = 1;
		let payload = PaginatedDeleteSignaturePayload {
			target_hash: PageHash::default(),
			msa_id,
			expiration: 10,
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = pair.sign(&encode_data_new_key_data).into();

		// act
		assert_err!(
			StatefulStoragePallet::delete_page_with_signature(
				RuntimeOrigin::signed(caller_1),
				delegator_key.into(),
				owner_signature,
				payload
			),
			Error::<Test>::InvalidMessageSourceAccount
		)
	})
}

#[test]
fn delete_page_with_signature_having_wrong_msa_in_payload_should_fail() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let (_correct_msa_id, pair) = get_signature_account();
		let delegator_key = pair.public();
		let wrong_msa_id = 3;
		let schema_id = UNDELEGATED_PAGINATED_SCHEMA;
		let page_id = 1;
		let payload = PaginatedDeleteSignaturePayload {
			target_hash: PageHash::default(),
			msa_id: wrong_msa_id,
			expiration: 10,
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = pair.sign(&encode_data_new_key_data).into();

		// act
		assert_err!(
			StatefulStoragePallet::delete_page_with_signature(
				RuntimeOrigin::signed(caller_1),
				delegator_key.into(),
				owner_signature,
				payload
			),
			Error::<Test>::InvalidMessageSourceAccount
		)
	})
}

#[test]
fn delete_page_with_signature_having_invalid_schema_id_should_fail() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let (msa_id, pair) = get_signature_account();
		let delegator_key = pair.public();
		let schema_id = INVALID_SCHEMA_ID;
		let page_id = 1;
		let payload = PaginatedDeleteSignaturePayload {
			target_hash: PageHash::default(),
			msa_id,
			expiration: 10,
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = pair.sign(&encode_data_new_key_data).into();

		// act
		assert_err!(
			StatefulStoragePallet::delete_page_with_signature(
				RuntimeOrigin::signed(caller_1),
				delegator_key.into(),
				owner_signature,
				payload
			),
			Error::<Test>::InvalidSchemaId
		)
	})
}

#[test]
fn delete_page_with_signature_having_invalid_schema_location_should_fail() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let (msa_id, pair) = get_signature_account();
		let delegator_key = pair.public();
		let schema_id = ITEMIZED_SCHEMA;
		let page_id = 1;
		let payload = PaginatedDeleteSignaturePayload {
			target_hash: PageHash::default(),
			msa_id,
			expiration: 10,
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = pair.sign(&encode_data_new_key_data).into();

		// act
		assert_err!(
			StatefulStoragePallet::delete_page_with_signature(
				RuntimeOrigin::signed(caller_1),
				delegator_key.into(),
				owner_signature,
				payload
			),
			Error::<Test>::SchemaPayloadLocationMismatch
		)
	})
}

#[test]
fn delete_page_with_signature_having_invalid_hash_should_fail() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let (msa_id, pair) = get_signature_account();
		let delegator_key = pair.public();
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 1;
		let payload = generate_payload_bytes::<PaginatedPageSize>(Some(100));
		assert_ok!(StatefulStoragePallet::upsert_page(
			RuntimeOrigin::signed(caller_1.clone()),
			msa_id,
			schema_id,
			page_id,
			NONEXISTENT_PAGE_HASH,
			payload.into(),
		));

		let payload = PaginatedDeleteSignaturePayload {
			target_hash: PageHash::default(),
			msa_id,
			expiration: 10,
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = pair.sign(&encode_data_new_key_data).into();

		// act
		assert_err!(
			StatefulStoragePallet::delete_page_with_signature(
				RuntimeOrigin::signed(caller_1),
				delegator_key.into(),
				owner_signature,
				payload
			),
			Error::<Test>::StalePageState
		)
	})
}

#[test]
fn delete_page_with_signature_with_non_existing_page_should_noop() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let (msa_id, pair) = get_signature_account();
		let delegator_key = pair.public();
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 1;
		let payload = PaginatedDeleteSignaturePayload {
			target_hash: PageHash::default(),
			msa_id,
			expiration: 10,
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = pair.sign(&encode_data_new_key_data).into();

		// act
		assert_storage_noop!(assert_eq!(
			StatefulStoragePallet::delete_page_with_signature(
				RuntimeOrigin::signed(caller_1),
				delegator_key.into(),
				owner_signature,
				payload
			),
			Ok(())
		));
	})
}

#[test]
fn delete_page_with_signature_having_valid_inputs_should_remove_page() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = test_public(1);
		let (msa_id, pair) = get_signature_account();
		let delegator_key = pair.public();
		let schema_id = PAGINATED_SCHEMA;
		let page_id = 1;
		let page = generate_page::<PaginatedPageSize>(Some(1), Some(100));
		<StatefulChildTree>::write(
			&msa_id,
			PALLET_STORAGE_PREFIX,
			PAGINATED_STORAGE_PREFIX,
			&(schema_id, page_id),
			&page,
		);

		let payload = PaginatedDeleteSignaturePayload {
			target_hash: page.get_hash(),
			msa_id,
			expiration: 10,
			schema_id,
			page_id,
		};
		let encode_data_new_key_data = wrap_binary_data(payload.encode());
		let owner_signature: MultiSignature = pair.sign(&encode_data_new_key_data).into();

		// act
		assert_ok!(StatefulStoragePallet::delete_page_with_signature(
			RuntimeOrigin::signed(caller_1),
			delegator_key.into(),
			owner_signature,
			payload
		));

		// assert
		let removed_page: Option<PaginatedPage<Test>> =
			StatefulChildTree::<<Test as Config>::KeyHasher>::try_read(
				&msa_id,
				PALLET_STORAGE_PREFIX,
				PAGINATED_STORAGE_PREFIX,
				&(schema_id, page_id),
			)
			.unwrap();
		assert!(removed_page.is_none());
		System::assert_last_event(
			StatefulEvent::PaginatedPageDeleted {
				msa_id,
				schema_id,
				page_id,
				prev_content_hash: page.get_hash(),
			}
			.into(),
		);
	})
}

#[test]
fn delete_page_fails_for_append_only() {
	new_test_ext().execute_with(|| {
		// setup
		let caller_1 = test_public(1);
		let msa_id = 1;
		let schema_id = PAGINATED_APPEND_ONLY_SCHEMA;
		let page_id = 11;
		let payload = generate_payload_bytes::<PaginatedPageSize>(None);
		let page: PaginatedPage<Test> = payload.clone().into();
		let page_hash = page.get_hash();

		// assert
		assert_ok!(StatefulStoragePallet::upsert_page(
			RuntimeOrigin::signed(caller_1.clone()),
			msa_id,
			schema_id,
			page_id,
			NONEXISTENT_PAGE_HASH,
			payload.into(),
		));

		assert_err!(
			StatefulStoragePallet::delete_page(
				RuntimeOrigin::signed(caller_1),
				msa_id,
				schema_id,
				page_id,
				page_hash
			),
			Error::<Test>::UnsupportedOperationForSchema
		);
	});
}
