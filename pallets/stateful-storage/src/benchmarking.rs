use super::*;
use crate::{types::ItemAction, Pallet as StatefulStoragePallet};
use common_primitives::schema::{ModelType, PayloadLocation};
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_core::bounded::BoundedVec;

fn itemized_actions_add<T: Config>(
	n: u32,
	s: usize,
) -> BoundedVec<ItemAction, T::MaxItemizedActionsCount> {
	let mut actions = vec![];
	for _ in 0..n {
		let payload = vec![0u8; s];
		actions.push(ItemAction::Add { data: payload.into() });
	}
	actions.try_into().expect("Invalid actions")
}

fn create_schema<T: Config>(location: PayloadLocation) -> DispatchResult {
	T::SchemaBenchmarkHelper::create_schema(
		Vec::from(r#"{"Message": "some-random-hash"}"#.as_bytes()),
		ModelType::AvroBinary,
		location,
	)
}

benchmarks! {
	apply_item_actions {
		let n in 0 .. T::MaxItemizedActionsCount::get() - 1;
		let s in 0 .. T::MaxItemizedBlobSizeBytes::get()- 1;
		let provider_msa_id = 1u64;
		let delegator_msa_id = 2u64;
		let schema_id = 1u16;
		let caller: T::AccountId = whitelisted_caller();
		let payload = vec![0u8; s as usize];

		assert_ok!(create_schema::<T>(PayloadLocation::OnChain)); // TODO change this to itemized
		assert_ok!(T::MsaBenchmarkHelper::add_key(provider_msa_id.into(), caller.clone()));
		assert_ok!(T::MsaBenchmarkHelper::set_delegation_relationship(provider_msa_id.into(), delegator_msa_id.into(), [schema_id].to_vec()));

		let actions = itemized_actions_add::<T>(n, s as usize);
	}: _ (RawOrigin::Signed(caller), delegator_msa_id.into(), schema_id, actions)
	verify {
		let page_result = StatefulStoragePallet::<T>::get_itemized_page(delegator_msa_id, schema_id);
		assert!(u32::from(page_result.item_count) == n);// TODO update with new changes 
	}

	impl_benchmark_test_suite!(StatefulStoragePallet,
		crate::mock::new_test_ext(),
		crate::mock::Test);
}
