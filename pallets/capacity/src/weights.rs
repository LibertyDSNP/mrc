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
//! DATE: 2023-11-30, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-runner-44wtw-5slfv`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

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
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `223`
		//  Estimated: `6249`
		// Minimum execution time: 44_240_000 picoseconds.
		Weight::from_parts(45_555_000, 6249)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:1)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:0)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn withdraw_unstaked() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `6249`
		// Minimum execution time: 28_294_000 picoseconds.
		Weight::from_parts(29_114_000, 6249)
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
		// Minimum execution time: 3_912_000 picoseconds.
		Weight::from_parts(4_081_000, 2974)
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
		//  Estimated: `5071`
		// Minimum execution time: 25_110_000 picoseconds.
		Weight::from_parts(25_886_000, 5071)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Capacity::EpochLength` (r:0 w:1)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_epoch_length() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_335_000 picoseconds.
		Weight::from_parts(6_662_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
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
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `223`
		//  Estimated: `6249`
		// Minimum execution time: 44_240_000 picoseconds.
		Weight::from_parts(45_555_000, 6249)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Capacity::UnstakeUnlocks` (r:1 w:1)
	/// Proof: `Capacity::UnstakeUnlocks` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
	/// Storage: `Capacity::StakingAccountLedger` (r:1 w:0)
	/// Proof: `Capacity::StakingAccountLedger` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn withdraw_unstaked() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `6249`
		// Minimum execution time: 28_294_000 picoseconds.
		Weight::from_parts(29_114_000, 6249)
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
		// Minimum execution time: 3_912_000 picoseconds.
		Weight::from_parts(4_081_000, 2974)
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
		//  Estimated: `5071`
		// Minimum execution time: 25_110_000 picoseconds.
		Weight::from_parts(25_886_000, 5071)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Capacity::EpochLength` (r:0 w:1)
	/// Proof: `Capacity::EpochLength` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_epoch_length() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_335_000 picoseconds.
		Weight::from_parts(6_662_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
