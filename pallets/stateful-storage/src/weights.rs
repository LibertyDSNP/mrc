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

//! Autogenerated weights for pallet_stateful_storage
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-02, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-runner-44wtw-n2ql6`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

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
// --additional-trie-layers=20
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/stateful-storage/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_stateful_storage.
pub trait WeightInfo {
	fn apply_item_actions(s: u32, ) -> Weight;
	fn upsert_page(s: u32, ) -> Weight;
	fn delete_page() -> Weight;
	fn apply_item_actions_with_signature(s: u32, ) -> Weight;
	fn upsert_page_with_signature(s: u32, ) -> Weight;
	fn delete_page_with_signature() -> Weight;
	fn apply_item_actions_test_pov(s: u32, ) -> Weight;
	fn upsert_page_test_pov(s: u32, ) -> Weight;
	fn delete_page_test_pov() -> Weight;
}

/// Weights for pallet_stateful_storage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1, 5121]`.
	fn apply_item_actions(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33346`
		//  Estimated: `45721`
		// Minimum execution time: 95_864_000 picoseconds.
		Weight::from_parts(100_554_019, 45721)
			// Standard Error: 615
			.saturating_add(Weight::from_parts(4_988, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `392`
		//  Estimated: `12767`
		// Minimum execution time: 28_918_000 picoseconds.
		Weight::from_parts(29_786_260, 12767)
			// Standard Error: 179
			.saturating_add(Weight::from_parts(963, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1551`
		//  Estimated: `13926`
		// Minimum execution time: 32_286_000 picoseconds.
		Weight::from_parts(33_306_000, 13926)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1, 5121]`.
	fn apply_item_actions_with_signature(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33353`
		//  Estimated: `45728`
		// Minimum execution time: 154_862_000 picoseconds.
		Weight::from_parts(149_197_324, 45728)
			// Standard Error: 483
			.saturating_add(Weight::from_parts(12_598, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page_with_signature(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `12700`
		// Minimum execution time: 82_069_000 picoseconds.
		Weight::from_parts(83_641_051, 12700)
			// Standard Error: 201
			.saturating_add(Weight::from_parts(6_617, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page_with_signature() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1484`
		//  Estimated: `13859`
		// Minimum execution time: 85_991_000 picoseconds.
		Weight::from_parts(86_655_000, 13859)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1, 5121]`.
	fn apply_item_actions_test_pov(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33346`
		//  Estimated: `45721`
		// Minimum execution time: 95_864_000 picoseconds.
		Weight::from_parts(100_554_019, 45721)
			// Standard Error: 615
			.saturating_add(Weight::from_parts(4_988, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page_test_pov(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `392`
		//  Estimated: `12767`
		// Minimum execution time: 28_918_000 picoseconds.
		Weight::from_parts(29_786_260, 12767)
			// Standard Error: 179
			.saturating_add(Weight::from_parts(963, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page_test_pov() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1551`
		//  Estimated: `13926`
		// Minimum execution time: 32_286_000 picoseconds.
		Weight::from_parts(33_306_000, 13926)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1, 5121]`.
	fn apply_item_actions(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33346`
		//  Estimated: `45721`
		// Minimum execution time: 95_864_000 picoseconds.
		Weight::from_parts(100_554_019, 45721)
			// Standard Error: 615
			.saturating_add(Weight::from_parts(4_988, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `392`
		//  Estimated: `12767`
		// Minimum execution time: 28_918_000 picoseconds.
		Weight::from_parts(29_786_260, 12767)
			// Standard Error: 179
			.saturating_add(Weight::from_parts(963, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1551`
		//  Estimated: `13926`
		// Minimum execution time: 32_286_000 picoseconds.
		Weight::from_parts(33_306_000, 13926)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1, 5121]`.
	fn apply_item_actions_with_signature(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33353`
		//  Estimated: `45728`
		// Minimum execution time: 154_862_000 picoseconds.
		Weight::from_parts(149_197_324, 45728)
			// Standard Error: 483
			.saturating_add(Weight::from_parts(12_598, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page_with_signature(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `12700`
		// Minimum execution time: 82_069_000 picoseconds.
		Weight::from_parts(83_641_051, 12700)
			// Standard Error: 201
			.saturating_add(Weight::from_parts(6_617, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page_with_signature() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1484`
		//  Estimated: `13859`
		// Minimum execution time: 85_991_000 picoseconds.
		Weight::from_parts(86_655_000, 13859)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1, 5121]`.
	fn apply_item_actions_test_pov(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33346`
		//  Estimated: `45721`
		// Minimum execution time: 95_864_000 picoseconds.
		Weight::from_parts(100_554_019, 45721)
			// Standard Error: 615
			.saturating_add(Weight::from_parts(4_988, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page_test_pov(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `392`
		//  Estimated: `12767`
		// Minimum execution time: 28_918_000 picoseconds.
		Weight::from_parts(29_786_260, 12767)
			// Standard Error: 179
			.saturating_add(Weight::from_parts(963, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page_test_pov() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1551`
		//  Estimated: `13926`
		// Minimum execution time: 32_286_000 picoseconds.
		Weight::from_parts(33_306_000, 13926)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
