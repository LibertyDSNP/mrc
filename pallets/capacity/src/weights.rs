// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_capacity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-05-09, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `UL-Mac.local`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

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

/// Weight functions needed for pallet_capacity.
pub trait WeightInfo {
	fn stake() -> Weight;
	fn withdraw_unstaked() -> Weight;
	fn on_initialize() -> Weight;
	fn unstake() -> Weight;
	fn set_epoch_length() -> Weight;
	fn change_staking_target() -> Weight;
	fn provider_boost() -> Weight;
}

/// Weights for pallet_capacity using the Substrate node and recommended hardware.
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
		//  Estimated: `6249`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(37_000_000, 6249)
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
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(24_000_000, 6249)
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
		//  Estimated: `2974`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_000_000, 2974)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingRewardPool` (r:1 w:1)
	/// Proof: `Capacity::StakingRewardPool` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:1)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `5071`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(29_000_000, 5071)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Capacity::EpochLength` (r:0 w:1)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_epoch_length() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
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
		//  Measured:  `315`
		//  Estimated: `7601`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 7601)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingRewardPool` (r:1 w:1)
	/// Proof: `Capacity::StakingRewardPool` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:0)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::ProviderBoostHistories` (r:1 w:1)
	/// Proof: `Capacity::ProviderBoostHistories` (`max_values`: None, `max_size`: Some(641), added: 3116, mode: `MaxEncodedLen`)
	fn provider_boost() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `247`
		//  Estimated: `6249`
		// Minimum execution time: 45_000_000 picoseconds.
		Weight::from_parts(46_000_000, 6249)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
}

// For backwards compatibility and tests
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
		//  Estimated: `6249`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(37_000_000, 6249)
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
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(24_000_000, 6249)
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
		//  Estimated: `2974`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_000_000, 2974)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingRewardPool` (r:1 w:1)
	/// Proof: `Capacity::StakingRewardPool` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:1)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `5071`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(29_000_000, 5071)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `Capacity::EpochLength` (r:0 w:1)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_epoch_length() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
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
		//  Measured:  `315`
		//  Estimated: `7601`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 7601)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingTargetLedger` (r:1 w:1)
	/// Proof: `Capacity::StakingTargetLedger` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::CapacityLedger` (r:1 w:1)
	/// Proof: `Capacity::CapacityLedger` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingRewardPool` (r:1 w:1)
	/// Proof: `Capacity::StakingRewardPool` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:0)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::ProviderBoostHistories` (r:1 w:1)
	/// Proof: `Capacity::ProviderBoostHistories` (`max_values`: None, `max_size`: Some(641), added: 3116, mode: `MaxEncodedLen`)
	fn provider_boost() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `247`
		//  Estimated: `6249`
		// Minimum execution time: 45_000_000 picoseconds.
		Weight::from_parts(46_000_000, 6249)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
}
