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

//! Autogenerated weights for pallet_handles
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-29, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-runner-44wtw-hp29g`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_handles
// --extrinsic
// *
// --chain=frequency-bench
// --execution=wasm
// --heap-pages=4096
// --wasm-execution=compiled
// --additional-trie-layers=20
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/handles/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_handles.
pub trait WeightInfo {
	fn claim_handle(b: u32, ) -> Weight;
	fn change_handle(b: u32, ) -> Weight;
	fn retire_handle() -> Weight;
}

/// Weights for pallet_handles using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Handles MSAIdToDisplayName (r:1 w:1)
	/// Proof: Handles MSAIdToDisplayName (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Handles CanonicalBaseHandleToSuffixIndex (r:1 w:1)
	/// Proof: Handles CanonicalBaseHandleToSuffixIndex (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Handles CanonicalBaseHandleAndSuffixToMSAId (r:0 w:1)
	/// Proof: Handles CanonicalBaseHandleAndSuffixToMSAId (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	/// The range of component `b` is `[3, 30]`.
	fn claim_handle(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `191`
		//  Estimated: `12434`
		// Minimum execution time: 81_138_000 picoseconds.
		Weight::from_parts(83_585_510, 12434)
			// Standard Error: 17_420
			.saturating_add(Weight::from_parts(54_544, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Handles MSAIdToDisplayName (r:1 w:1)
	/// Proof: Handles MSAIdToDisplayName (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Handles CanonicalBaseHandleToSuffixIndex (r:1 w:1)
	/// Proof: Handles CanonicalBaseHandleToSuffixIndex (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Handles CanonicalBaseHandleAndSuffixToMSAId (r:0 w:2)
	/// Proof: Handles CanonicalBaseHandleAndSuffixToMSAId (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	/// The range of component `b` is `[3, 30]`.
	fn change_handle(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `297 + b * (1 ±0)`
		//  Estimated: `12434`
		// Minimum execution time: 91_500_000 picoseconds.
		Weight::from_parts(93_320_148, 12434)
			// Standard Error: 10_602
			.saturating_add(Weight::from_parts(152_480, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Handles MSAIdToDisplayName (r:1 w:1)
	/// Proof: Handles MSAIdToDisplayName (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Handles CanonicalBaseHandleAndSuffixToMSAId (r:0 w:1)
	/// Proof: Handles CanonicalBaseHandleAndSuffixToMSAId (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	fn retire_handle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306`
		//  Estimated: `12434`
		// Minimum execution time: 21_879_000 picoseconds.
		Weight::from_parts(22_500_000, 12434)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Handles MSAIdToDisplayName (r:1 w:1)
	/// Proof: Handles MSAIdToDisplayName (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Handles CanonicalBaseHandleToSuffixIndex (r:1 w:1)
	/// Proof: Handles CanonicalBaseHandleToSuffixIndex (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Handles CanonicalBaseHandleAndSuffixToMSAId (r:0 w:1)
	/// Proof: Handles CanonicalBaseHandleAndSuffixToMSAId (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	/// The range of component `b` is `[3, 30]`.
	fn claim_handle(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `191`
		//  Estimated: `12434`
		// Minimum execution time: 81_138_000 picoseconds.
		Weight::from_parts(83_585_510, 12434)
			// Standard Error: 17_420
			.saturating_add(Weight::from_parts(54_544, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Handles MSAIdToDisplayName (r:1 w:1)
	/// Proof: Handles MSAIdToDisplayName (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Handles CanonicalBaseHandleToSuffixIndex (r:1 w:1)
	/// Proof: Handles CanonicalBaseHandleToSuffixIndex (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Handles CanonicalBaseHandleAndSuffixToMSAId (r:0 w:2)
	/// Proof: Handles CanonicalBaseHandleAndSuffixToMSAId (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	/// The range of component `b` is `[3, 30]`.
	fn change_handle(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `297 + b * (1 ±0)`
		//  Estimated: `12434`
		// Minimum execution time: 91_500_000 picoseconds.
		Weight::from_parts(93_320_148, 12434)
			// Standard Error: 10_602
			.saturating_add(Weight::from_parts(152_480, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Handles MSAIdToDisplayName (r:1 w:1)
	/// Proof: Handles MSAIdToDisplayName (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Handles CanonicalBaseHandleAndSuffixToMSAId (r:0 w:1)
	/// Proof: Handles CanonicalBaseHandleAndSuffixToMSAId (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	fn retire_handle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306`
		//  Estimated: `12434`
		// Minimum execution time: 21_879_000 picoseconds.
		Weight::from_parts(22_500_000, 12434)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
