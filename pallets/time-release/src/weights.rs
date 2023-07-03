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

//! Autogenerated weights for pallet_time_release
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-03, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-runner-qpqf8-9nfdk`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_time-release
// --extrinsic
// *
// --chain=frequency-bench
// --execution=wasm
// --heap-pages=4096
// --wasm-execution=compiled
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/time-release/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_time_release.
pub trait WeightInfo {
	fn transfer() -> Weight;
	fn claim(i: u32, ) -> Weight;
	fn update_release_schedules(i: u32, ) -> Weight;
}

/// Weights for pallet_time_release using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TimeRelease ReleaseSchedules (r:1 w:1)
	/// Proof: TimeRelease ReleaseSchedules (max_values: None, max_size: Some(1449), added: 3924, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `4914`
		// Minimum execution time: 16_619_000 picoseconds.
		Weight::from_parts(16_991_000, 4914)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TimeRelease ReleaseSchedules (r:1 w:1)
	/// Proof: TimeRelease ReleaseSchedules (max_values: None, max_size: Some(1449), added: 3924, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 50]`.
	fn claim(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `201`
		//  Estimated: `4914`
		// Minimum execution time: 31_370_000 picoseconds.
		Weight::from_parts(32_518_337, 4914)
			// Standard Error: 2_092
			.saturating_add(Weight::from_parts(4_424, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: TimeRelease ReleaseSchedules (r:0 w:1)
	/// Proof: TimeRelease ReleaseSchedules (max_values: None, max_size: Some(1449), added: 3924, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 50]`.
	fn update_release_schedules(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `39`
		//  Estimated: `3593`
		// Minimum execution time: 11_352_000 picoseconds.
		Weight::from_parts(12_361_033, 3593)
			// Standard Error: 1_831
			.saturating_add(Weight::from_parts(68_027, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TimeRelease ReleaseSchedules (r:1 w:1)
	/// Proof: TimeRelease ReleaseSchedules (max_values: None, max_size: Some(1449), added: 3924, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `4914`
		// Minimum execution time: 16_619_000 picoseconds.
		Weight::from_parts(16_991_000, 4914)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TimeRelease ReleaseSchedules (r:1 w:1)
	/// Proof: TimeRelease ReleaseSchedules (max_values: None, max_size: Some(1449), added: 3924, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 50]`.
	fn claim(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `201`
		//  Estimated: `4914`
		// Minimum execution time: 31_370_000 picoseconds.
		Weight::from_parts(32_518_337, 4914)
			// Standard Error: 2_092
			.saturating_add(Weight::from_parts(4_424, 0).saturating_mul(i.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: TimeRelease ReleaseSchedules (r:0 w:1)
	/// Proof: TimeRelease ReleaseSchedules (max_values: None, max_size: Some(1449), added: 3924, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 50]`.
	fn update_release_schedules(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `39`
		//  Estimated: `3593`
		// Minimum execution time: 11_352_000 picoseconds.
		Weight::from_parts(12_361_033, 3593)
			// Standard Error: 1_831
			.saturating_add(Weight::from_parts(68_027, 0).saturating_mul(i.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
