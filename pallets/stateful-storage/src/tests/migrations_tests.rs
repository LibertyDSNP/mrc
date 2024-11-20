use crate::{
	migration::v1,
	test_common::constants::ITEMIZED_APPEND_ONLY_SCHEMA,
	tests::mock::{new_test_ext, test_public, RuntimeOrigin, StatefulStoragePallet, Test},
	types::ItemAction,
};
use common_primitives::stateful_storage::PageHash;
use frame_support::{
	assert_ok, pallet_prelude::StorageVersion, traits::GetStorageVersion, BoundedVec,
};

#[test]
fn migration_to_v1_should_work_as_expected() {
	new_test_ext().execute_with(|| {
		// arrange
		let msa_id = 1;
		let caller_1 = test_public(msa_id);
		let schema_id = ITEMIZED_APPEND_ONLY_SCHEMA;
		let payload =
			hex::decode("0e0abc9afdfb34cf30c59d16bbbbd76cae1668fc331eb9e505cb0d6af6365f17")
				.expect("should decode hex");
		let expected =
			hex::decode("400e0abc9afdfb34cf30c59d16bbbbd76cae1668fc331eb9e505cb0d6af6365f17")
				.expect("should decode hex");
		let prev_content_hash: PageHash = 0;
		let actions = vec![ItemAction::Add { data: payload.try_into().unwrap() }];

		// act
		assert_ok!(StatefulStoragePallet::apply_item_actions(
			RuntimeOrigin::signed(caller_1),
			msa_id,
			schema_id,
			prev_content_hash,
			BoundedVec::try_from(actions).unwrap(),
		));

		// Act
		let _ = v1::migrate_to_v1::<Test>();

		// Assert
		let current_version = StatefulStoragePallet::on_chain_storage_version();
		assert_eq!(current_version, StorageVersion::new(1));

		let known_msa_ids = v1::get_msa_ids::<Test>();
		assert_eq!(known_msa_ids.len(), 1);

		let after_update =
			StatefulStoragePallet::get_itemized_storage(msa_id, schema_id).expect("should get");
		let item = after_update.items.into_iter().next().expect("should item exists");
		assert_eq!(item.payload, expected);
	});
}
