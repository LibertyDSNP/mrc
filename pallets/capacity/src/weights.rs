
//! Autogenerated weights for `pallet_capacity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-07-22, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-10-212`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("frequency-bench")`, DB CACHE: `1024`

// Executed Command:
// ./scripts/../target/bench-dev/frequency
// benchmark
// pallet
// --pallet=pallet_capacity
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --additional-trie-layers=5
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/capacity/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

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
	fn start_new_epoch_if_needed() -> Weight;
	fn start_new_reward_era_if_needed() -> Weight;
	fn unstake() -> Weight;
	fn set_epoch_length() -> Weight;
	fn change_staking_target() -> Weight;
	fn provider_boost() -> Weight;
	fn claim_staking_rewards() -> Weight;
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
		//  Measured:  `177`
		//  Estimated: `6249`
		// Minimum execution time: 39_381_000 picoseconds.
		Weight::from_parts(40_678_000, 6249)
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
		//  Measured:  `226`
		//  Estimated: `6249`
		// Minimum execution time: 27_182_000 picoseconds.
		Weight::from_parts(27_758_000, 6249)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Capacity::CurrentEpochInfo` (r:1 w:1)
	/// Proof: `Capacity::CurrentEpochInfo` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::EpochLength` (r:1 w:0)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn start_new_epoch_if_needed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `2974`
		// Minimum execution time: 2_302_000 picoseconds.
		Weight::from_parts(2_416_000, 2974)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Capacity::CurrentEraProviderBoostTotal` (r:1 w:0)
	/// Proof: `Capacity::CurrentEraProviderBoostTotal` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::ProviderBoostRewardPools` (r:1 w:1)
	/// Proof: `Capacity::ProviderBoostRewardPools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn start_new_reward_era_if_needed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `193`
		//  Estimated: `5063`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(5_000_000, 5063)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Capacity::ProviderBoostHistories` (r:1 w:1)
	/// Proof: `Capacity::ProviderBoostHistories` (`max_values`: None, `max_size`: Some(641), added: 3116, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CurrentEraProviderBoostTotal` (r:1 w:1)
	/// Proof: `Capacity::CurrentEraProviderBoostTotal` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:1)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `359`
		//  Estimated: `5591`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 5591)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Capacity::EpochLength` (r:0 w:1)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_epoch_length() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_067_000 picoseconds.
		Weight::from_parts(4_482_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Capacity::Retargets` (r:1 w:1)
	/// Proof: `Capacity::Retargets` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:0)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:2 w:2)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:2 w:2)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn change_staking_target() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `353`
		//  Estimated: `7601`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(29_000_000, 7601)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:0)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::ProviderBoostHistories` (r:1 w:1)
	/// Proof: `Capacity::ProviderBoostHistories` (`max_values`: None, `max_size`: Some(641), added: 3116, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CurrentEraProviderBoostTotal` (r:1 w:1)
	/// Proof: `Capacity::CurrentEraProviderBoostTotal` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn provider_boost() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177`
		//  Estimated: `6249`
		// Minimum execution time: 37_000_000 picoseconds.
		Weight::from_parts(38_000_000, 6249)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Capacity::ProviderBoostHistories` (r:1 w:1)
	/// Proof: `Capacity::ProviderBoostHistories` (`max_values`: None, `max_size`: Some(641), added: 3116, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::ProviderBoostRewardPools` (r:7 w:0)
	/// Proof: `Capacity::ProviderBoostRewardPools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn claim_staking_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1592`
		//  Estimated: `20591`
		// Minimum execution time: 104_000_000 picoseconds.
		Weight::from_parts(105_000_000, 20591)
			.saturating_add(T::DbWeight::get().reads(8_u64))
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
		//  Measured:  `177`
		//  Estimated: `6249`
		// Minimum execution time: 39_381_000 picoseconds.
		Weight::from_parts(40_678_000, 6249)
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
		//  Measured:  `226`
		//  Estimated: `6249`
		// Minimum execution time: 27_182_000 picoseconds.
		Weight::from_parts(27_758_000, 6249)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Capacity::CurrentEpochInfo` (r:1 w:1)
	/// Proof: `Capacity::CurrentEpochInfo` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::EpochLength` (r:1 w:0)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn start_new_epoch_if_needed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `2974`
		// Minimum execution time: 1_000_000 picoseconds.
		Weight::from_parts(2_000_000, 2974)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Capacity::CurrentEraProviderBoostTotal` (r:1 w:0)
	/// Proof: `Capacity::CurrentEraProviderBoostTotal` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::ProviderBoostRewardPools` (r:1 w:1)
	/// Proof: `Capacity::ProviderBoostRewardPools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn start_new_reward_era_if_needed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `193`
		//  Estimated: `5063`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(5_000_000, 5063)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Capacity::ProviderBoostHistories` (r:1 w:1)
	/// Proof: `Capacity::ProviderBoostHistories` (`max_values`: None, `max_size`: Some(641), added: 3116, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CurrentEraProviderBoostTotal` (r:1 w:1)
	/// Proof: `Capacity::CurrentEraProviderBoostTotal` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:1)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `359`
		//  Estimated: `5591`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 5591)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Capacity::EpochLength` (r:0 w:1)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_epoch_length() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_067_000 picoseconds.
		Weight::from_parts(4_482_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Capacity::Retargets` (r:1 w:1)
	/// Proof: `Capacity::Retargets` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:0)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:2 w:2)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:2 w:2)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn change_staking_target() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `353`
		//  Estimated: `7601`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(29_000_000, 7601)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:0)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::ProviderBoostHistories` (r:1 w:1)
	/// Proof: `Capacity::ProviderBoostHistories` (`max_values`: None, `max_size`: Some(641), added: 3116, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CurrentEraProviderBoostTotal` (r:1 w:1)
	/// Proof: `Capacity::CurrentEraProviderBoostTotal` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn provider_boost() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177`
		//  Estimated: `6249`
		// Minimum execution time: 37_000_000 picoseconds.
		Weight::from_parts(38_000_000, 6249)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Capacity::ProviderBoostHistories` (r:1 w:1)
	/// Proof: `Capacity::ProviderBoostHistories` (`max_values`: None, `max_size`: Some(641), added: 3116, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::ProviderBoostRewardPools` (r:7 w:0)
	/// Proof: `Capacity::ProviderBoostRewardPools` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn claim_staking_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1592`
		//  Estimated: `20591`
		// Minimum execution time: 104_000_000 picoseconds.
		Weight::from_parts(105_000_000, 20591)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}


#[cfg(test)]
mod tests {
  use frame_support::{traits::Get, weights::Weight, dispatch::DispatchClass};
  use common_runtime::constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
  use common_runtime::weights::extrinsic_weights::ExtrinsicBaseWeight;

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
				> 6249
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
				> 6249
		);
	}
	#[test]
	fn test_start_new_epoch_if_needed() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 2974
		);
	}
	#[test]
	fn test_start_new_reward_era_if_needed() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5063
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
				> 5591
		);
	}
	#[test]
	fn test_change_staking_target() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7601
		);
	}
	#[test]
	fn test_provider_boost() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6249
		);
	}
	#[test]
	fn test_claim_staking_rewards() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 20591
		);
	}
}
