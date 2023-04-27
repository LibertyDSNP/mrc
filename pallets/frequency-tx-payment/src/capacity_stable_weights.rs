//! Fixes the Weight values for Capacity transactions as static values
//! Any change in actual weight does not adjust the cost, but will still adjust the block space
//!

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(
	rustdoc::all,
	missing_docs,
	unused_parens,
	unused_imports
)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_msa.
pub trait WeightInfo {
	// MSA
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight;
	fn add_public_key_to_msa() -> Weight;
	fn grant_delegation(s: u32, ) -> Weight;
	fn grant_schema_permissions(s: u32, ) -> Weight;
	// Messages
	fn add_onchain_message(n: u32, ) -> Weight;
	fn add_ipfs_message() -> Weight;
	// Stateful-storage
	fn apply_item_actions(n: u32, ) -> Weight;
	fn upsert_page(s: u32, ) -> Weight;
	fn delete_page() -> Weight;
	fn apply_item_actions_with_signature(s: u32, ) -> Weight;
	fn upsert_page_with_signature(s: u32, ) -> Weight;
	fn delete_page_with_signature() -> Weight;
	// Handles
	fn claim_handle(b: u32, ) -> Weight;
}

// Update test as well to ensure static weight values `tests/stable_weights_test.rs`

/// Weights for pallet_msa using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Msa PayloadSignatureBucketCount (r:1 w:1)
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa CurrentMsaIdentifierMaximum (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight {
		Weight::from_ref_time(100_556_500 as u64)
			// Standard Error: 19_778
			.saturating_add(Weight::from_ref_time(120_447 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Msa PayloadSignatureBucketCount (r:1 w:1)
	// Storage: Msa PayloadSignatureRegistry (r:2 w:2)
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn add_public_key_to_msa() -> Weight {
		Weight::from_ref_time(147_786_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Msa PayloadSignatureBucketCount (r:1 w:1)
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:0)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn grant_delegation(s: u32, ) -> Weight {
		Weight::from_ref_time(94_743_045 as u64)
			// Standard Error: 19_748
			.saturating_add(Weight::from_ref_time(125_241 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn grant_schema_permissions(s: u32, ) -> Weight {
		Weight::from_ref_time(26_682_873 as u64)
			// Standard Error: 7_236
			.saturating_add(Weight::from_ref_time(63_887 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
		// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: Messages Messages (r:1 w:1)
	fn add_onchain_message(n: u32, ) -> Weight {
		Weight::from_ref_time(139_432_286 as u64)
			// Standard Error: 43
			.saturating_add(Weight::from_ref_time(1_441 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Messages Messages (r:1 w:1)
	fn add_ipfs_message() -> Weight {
		Weight::from_ref_time(131_669_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: unknown [0xbd1557c8db6bd8599a811a7175fbc2fc6400] (r:1 w:1)
	fn apply_item_actions(s: u32, ) -> Weight {
		Weight::from_ref_time(66_026_301 as u64)
			// Standard Error: 161
			.saturating_add(Weight::from_ref_time(2_145 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: unknown [0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1] (r:1 w:1)
	fn upsert_page(s: u32, ) -> Weight {
		Weight::from_ref_time(23_029_186 as u64)
			// Standard Error: 53
			.saturating_add(Weight::from_ref_time(339 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: unknown [0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1] (r:1 w:1)
	fn delete_page() -> Weight {
		Weight::from_ref_time(26_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: unknown [0xbd1557c8db6bd8599a811a7175fbc2fc6400] (r:1 w:1)
	fn apply_item_actions_with_signature(s: u32, ) -> Weight {
		Weight::from_ref_time(105_921_191 as u64)
			// Standard Error: 267
			.saturating_add(Weight::from_ref_time(6_150 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: unknown [0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1] (r:1 w:1)
	fn upsert_page_with_signature(s: u32, ) -> Weight {
		Weight::from_ref_time(61_324_707 as u64)
			// Standard Error: 249
			.saturating_add(Weight::from_ref_time(4_406 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: unknown [0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1] (r:1 w:1)
	fn delete_page_with_signature() -> Weight {
		Weight::from_ref_time(65_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Handles MSAIdToDisplayName (r:1 w:1)
	// Storage: Handles CanonicalBaseHandleToSuffixIndex (r:1 w:1)
	// Storage: Handles CanonicalBaseHandleAndSuffixToMSAId (r:0 w:1)
	fn claim_handle(b: u32, ) -> Weight {
		Weight::from_ref_time(90_537_753 as u64)
			// Standard Error: 27_078
			.saturating_add(Weight::from_ref_time(104_522 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
