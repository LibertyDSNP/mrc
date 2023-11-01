#[cfg(feature = "try-runtime")]
use crate::types::SCHEMA_STORAGE_VERSION;
use crate::{
	pallet::{SchemaInfos, SchemaPayloads},
	Config, Pallet, SchemaId, SchemaInfo, LOG_TARGET,
};
use frame_support::{
	log, pallet_prelude::*, storage_alias, traits::OnRuntimeUpgrade, weights::Weight,
};
use sp_runtime::Saturating;
#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

/// old module storages
pub mod old {
	use super::*;
	use crate::Schema;
	/// Storage for message schema struct data
	/// - Key: Schema Id
	/// - Value: [`Schema`](Schema)
	#[storage_alias]
	pub(crate) type Schemas<T: Config> = StorageMap<
		Pallet<T>,
		Twox64Concat,
		SchemaId,
		Schema<<T as crate::pallet::Config>::SchemaModelMaxBytesBoundedVecLimit>,
		OptionQuery,
	>;
}

/// migration to v2 implementation
pub struct MigrateToV2<T>(PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
	fn on_runtime_upgrade() -> Weight {
		migrate_to_v2::<T>()
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
		log::info!(target: LOG_TARGET, "Running pre_upgrade...");
		let count = old::Schemas::<T>::iter().count() as u32;
		log::info!(target: LOG_TARGET, "Finish pre_upgrade for {:?}", count);
		Ok(count.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), TryRuntimeError> {
		log::info!(target: LOG_TARGET, "Running post_upgrade...");

		let old_count: u32 = Decode::decode(&mut state.as_slice())
			.expect("the state parameter should be something that was generated by pre_upgrade");

		let current_count = old::Schemas::<T>::iter().count();
		let info_count = SchemaInfos::<T>::iter().count();
		let payload_count = SchemaPayloads::<T>::iter().count();

		log::info!(target: LOG_TARGET, "Finish post_upgrade for {:?}", info_count);
		let onchain_version = Pallet::<T>::on_chain_storage_version();

		assert_eq!(current_count, 0usize);
		assert_eq!(info_count, old_count as usize);
		assert_eq!(payload_count, old_count as usize);
		assert_eq!(onchain_version, SCHEMA_STORAGE_VERSION);
		Ok(())
	}
}

/// migrating to v2
pub fn migrate_to_v2<T: Config>() -> Weight {
	log::info!(target: LOG_TARGET, "Running storage migration...");
	let onchain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();
	log::info!(target: LOG_TARGET, "onchain_version= {:?}, current_version={:?}", onchain_version, current_version);

	if onchain_version < 2 {
		let mut reads = 1u64;
		let mut writes = 0u64;
		let mut bytes = 0u64;
		for (schema_id, schema) in old::Schemas::<T>::drain() {
			bytes = bytes.saturating_add(schema.encode().len() as u64);

			let info = SchemaInfo {
				model_type: schema.model_type,
				payload_location: schema.payload_location,
				settings: schema.settings,
			};

			bytes = bytes.saturating_add(info.encode().len() as u64);
			SchemaInfos::<T>::insert(schema_id, info);

			bytes = bytes.saturating_add(schema.model.len() as u64);
			SchemaPayloads::<T>::insert(schema_id, schema.model);

			reads.saturating_inc();
			writes = writes.saturating_add(3);
		}

		// Set storage version to `2`.
		StorageVersion::new(2).put::<Pallet<T>>();
		writes.saturating_inc();

		log::info!(target: LOG_TARGET, "Storage migrated to version 2  read={:?}, write={:?}, bytes={:?}", reads, writes, bytes);
		let weights = T::DbWeight::get().reads_writes(reads, writes).add_proof_size(bytes);
		log::info!(target: LOG_TARGET, "Migration Calculated weights={:?}",weights);
		weights
	} else {
		log::info!(
			target: LOG_TARGET,
			"Migration did not execute. This probably should be removed onchain:{:?}, current:{:?}",
			onchain_version,
			current_version
		);
		T::DbWeight::get().reads(1)
	}
}
