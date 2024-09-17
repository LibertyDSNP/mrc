
//! Autogenerated weights for `pallet_capacity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-09-17, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-4-60`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("frequency-bench")`, DB CACHE: `1024`

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_capacity
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/capacity/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs
// --additional-trie-layers=3

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_capacity`.
pub trait WeightInfo {
	fn stake() -> Weight;
	fn withdraw_unstaked() -> Weight;
	fn on_initialize() -> Weight;
	fn unstake() -> Weight;
	fn set_epoch_length() -> Weight;
}

/// Weights for `pallet_capacity` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:0)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	fn stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174`
		//  Estimated: `5259`
		// Minimum execution time: 37_899_000 picoseconds.
		Weight::from_parts(39_145_000, 5259)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:1)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:0)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	fn withdraw_unstaked() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `5259`
		// Minimum execution time: 25_067_000 picoseconds.
		Weight::from_parts(25_710_000, 5259)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Capacity::CurrentEpochInfo` (r:1 w:1)
	/// Proof: `Capacity::CurrentEpochInfo` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::EpochLength` (r:1 w:0)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `1984`
		// Minimum execution time: 2_174_000 picoseconds.
		Weight::from_parts(2_346_000, 1984)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:1)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `271`
		//  Estimated: `4081`
		// Minimum execution time: 24_127_000 picoseconds.
		Weight::from_parts(24_919_000, 4081)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Capacity::EpochLength` (r:0 w:1)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_epoch_length() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_241_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:0)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	fn stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174`
		//  Estimated: `5259`
		// Minimum execution time: 37_899_000 picoseconds.
		Weight::from_parts(39_145_000, 5259)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:1)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:0)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	fn withdraw_unstaked() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `5259`
		// Minimum execution time: 25_067_000 picoseconds.
		Weight::from_parts(25_710_000, 5259)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Capacity::CurrentEpochInfo` (r:1 w:1)
	/// Proof: `Capacity::CurrentEpochInfo` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::EpochLength` (r:1 w:0)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `1984`
		// Minimum execution time: 2_174_000 picoseconds.
		Weight::from_parts(2_346_000, 1984)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:1)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `271`
		//  Estimated: `4081`
		// Minimum execution time: 24_127_000 picoseconds.
		Weight::from_parts(24_919_000, 4081)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Capacity::EpochLength` (r:0 w:1)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_epoch_length() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(4_241_000, 0)
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
	fn test_stake() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5259
		);
	}
	#[test]
	fn test_withdraw_unstaked() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5259
		);
	}
	#[test]
	fn test_on_initialize() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 1984
		);
	}
	#[test]
	fn test_unstake() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4081
		);
	}
}
