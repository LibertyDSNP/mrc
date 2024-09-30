
//! Autogenerated weights for `pallet_stateful_storage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-09-30, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-4-65`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("frequency-bench")`, DB CACHE: `1024`

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_stateful-storage
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/stateful-storage/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs
// --additional-trie-layers=6

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_stateful_storage`.
pub trait WeightInfo {
	fn apply_item_actions_add(s: u32, ) -> Weight;
	fn apply_item_actions_delete(n: u32, ) -> Weight;
	fn upsert_page(s: u32, ) -> Weight;
	fn delete_page() -> Weight;
	fn apply_item_actions_with_signature_v2_add(s: u32, ) -> Weight;
	fn apply_item_actions_with_signature_v2_delete(n: u32, ) -> Weight;
	fn upsert_page_with_signature_v2(s: u32, ) -> Weight;
	fn delete_page_with_signature_v2() -> Weight;
}

/// Weights for `pallet_stateful_storage` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1024, 5120]`.
	fn apply_item_actions_add(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `318`
		//  Estimated: `5763`
		// Minimum execution time: 24_937_000 picoseconds.
		Weight::from_parts(24_830_856, 5763)
			// Standard Error: 54
			.saturating_add(Weight::from_parts(977, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn apply_item_actions_delete(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10696`
		//  Estimated: `16141`
		// Minimum execution time: 39_102_000 picoseconds.
		Weight::from_parts(42_522_350, 16141)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1479`
		//  Estimated: `6924`
		// Minimum execution time: 28_794_000 picoseconds.
		Weight::from_parts(29_990_443, 6924)
			// Standard Error: 182
			.saturating_add(Weight::from_parts(899, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1477`
		//  Estimated: `6922`
		// Minimum execution time: 28_169_000 picoseconds.
		Weight::from_parts(29_093_000, 6922)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1024, 5120]`.
	fn apply_item_actions_with_signature_v2_add(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `5770`
		// Minimum execution time: 71_194_000 picoseconds.
		Weight::from_parts(64_537_378, 5770)
			// Standard Error: 182
			.saturating_add(Weight::from_parts(6_399, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn apply_item_actions_with_signature_v2_delete(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10703`
		//  Estimated: `16148`
		// Minimum execution time: 78_332_000 picoseconds.
		Weight::from_parts(84_183_512, 16148)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page_with_signature_v2(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1412`
		//  Estimated: `6857`
		// Minimum execution time: 68_313_000 picoseconds.
		Weight::from_parts(70_263_595, 6857)
			// Standard Error: 488
			.saturating_add(Weight::from_parts(7_032, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page_with_signature_v2() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1410`
		//  Estimated: `6855`
		// Minimum execution time: 67_493_000 picoseconds.
		Weight::from_parts(68_573_000, 6855)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1024, 5120]`.
	fn apply_item_actions_add(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `318`
		//  Estimated: `5763`
		// Minimum execution time: 24_937_000 picoseconds.
		Weight::from_parts(24_830_856, 5763)
			// Standard Error: 54
			.saturating_add(Weight::from_parts(977, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn apply_item_actions_delete(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10696`
		//  Estimated: `16141`
		// Minimum execution time: 39_102_000 picoseconds.
		Weight::from_parts(42_522_350, 16141)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1479`
		//  Estimated: `6924`
		// Minimum execution time: 28_794_000 picoseconds.
		Weight::from_parts(29_990_443, 6924)
			// Standard Error: 182
			.saturating_add(Weight::from_parts(899, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1477`
		//  Estimated: `6922`
		// Minimum execution time: 28_169_000 picoseconds.
		Weight::from_parts(29_093_000, 6922)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1024, 5120]`.
	fn apply_item_actions_with_signature_v2_add(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `5770`
		// Minimum execution time: 71_194_000 picoseconds.
		Weight::from_parts(64_537_378, 5770)
			// Standard Error: 182
			.saturating_add(Weight::from_parts(6_399, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn apply_item_actions_with_signature_v2_delete(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10703`
		//  Estimated: `16148`
		// Minimum execution time: 78_332_000 picoseconds.
		Weight::from_parts(84_183_512, 16148)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page_with_signature_v2(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1412`
		//  Estimated: `6857`
		// Minimum execution time: 68_313_000 picoseconds.
		Weight::from_parts(70_263_595, 6857)
			// Standard Error: 488
			.saturating_add(Weight::from_parts(7_032, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page_with_signature_v2() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1410`
		//  Estimated: `6855`
		// Minimum execution time: 67_493_000 picoseconds.
		Weight::from_parts(68_573_000, 6855)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}


#[cfg(test)]
mod tests {
  use frame_support::{traits::Get, weights::Weight, dispatch::DispatchClass};
  use common_runtime::constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
  use common_runtime::weights::extrinsic_weights::ExtrinsicBaseWeight;

  #[allow(dead_code)]
  struct BlockWeights;
  impl Get<frame_system::limits::BlockWeights> for BlockWeights {
  	fn get() -> frame_system::limits::BlockWeights {
  		frame_system::limits::BlockWeights::builder()
  			.base_block(Weight::zero())
  			.for_class(DispatchClass::all(), |weights| {
  				weights.base_extrinsic = ExtrinsicBaseWeight::get().into();
  			})
  			.for_class(DispatchClass::non_mandatory(), |weights| {
  				weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
  			})
  			.build_or_panic()
  	}
  }

	#[test]
	fn test_apply_item_actions_add() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5763
		);
	}
	#[test]
	fn test_apply_item_actions_delete() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 16141
		);
	}
	#[test]
	fn test_upsert_page() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6924
		);
	}
	#[test]
	fn test_delete_page() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6922
		);
	}
	#[test]
	fn test_apply_item_actions_with_signature_v2_add() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5770
		);
	}
	#[test]
	fn test_apply_item_actions_with_signature_v2_delete() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 16148
		);
	}
	#[test]
	fn test_upsert_page_with_signature_v2() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6857
		);
	}
	#[test]
	fn test_delete_page_with_signature_v2() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6855
		);
	}
}
