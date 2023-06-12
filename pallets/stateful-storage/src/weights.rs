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
//! DATE: 2023-06-12, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_stateful-storage
// --extrinsic
// *
// --chain=frequency-bench
// --execution=wasm
// --heap-pages=4096
// --wasm-execution=compiled
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/stateful-storage/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(
	rustdoc::all,
	missing_docs,
	unused_parens,
	unused_imports
)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_stateful_storage.
pub trait WeightInfo {
	fn apply_item_actions(s: u32, ) -> Weight;
	fn upsert_page(s: u32, ) -> Weight;
	fn delete_page() -> Weight;
	fn apply_item_actions_with_signature(s: u32, ) -> Weight;
	fn upsert_page_with_signature(s: u32, ) -> Weight;
	fn delete_page_with_signature() -> Weight;
}

/// Weights for pallet_stateful_storage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	// Storage: unknown `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	// Proof Skipped: unknown `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	fn apply_item_actions(s: u32, ) -> Weight {
		Weight::from_parts(106_673_188 as u64, 0)
			// Standard Error: 335
			.saturating_add(Weight::from_parts(7_546 as u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	// Storage: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	// Proof Skipped: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn upsert_page(s: u32, ) -> Weight {
		Weight::from_parts(27_745_218 as u64, 0)
			// Standard Error: 1_758
			.saturating_add(Weight::from_parts(16_091 as u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	// Storage: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	// Proof Skipped: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page() -> Weight {
		Weight::from_parts(39_362_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: unknown `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	// Proof Skipped: unknown `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	fn apply_item_actions_with_signature(s: u32, ) -> Weight {
		Weight::from_parts(162_349_356 as u64, 0)
			// Standard Error: 436
			.saturating_add(Weight::from_parts(13_673 as u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	// Proof Skipped: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn upsert_page_with_signature(s: u32, ) -> Weight {
		Weight::from_parts(87_513_351 as u64, 0)
			// Standard Error: 394
			.saturating_add(Weight::from_parts(5_610 as u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	// Proof Skipped: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page_with_signature() -> Weight {
		Weight::from_parts(90_185_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	// Storage: unknown `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	// Proof Skipped: unknown `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	fn apply_item_actions(s: u32, ) -> Weight {
		Weight::from_parts(106_673_188 as u64, 0)
			// Standard Error: 335
			.saturating_add(Weight::from_parts(7_546 as u64, 0).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	// Storage: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	// Proof Skipped: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn upsert_page(s: u32, ) -> Weight {
		Weight::from_parts(27_745_218 as u64, 0)
			// Standard Error: 1_758
			.saturating_add(Weight::from_parts(16_091 as u64, 0).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	// Storage: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	// Proof Skipped: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page() -> Weight {
		Weight::from_parts(39_362_000 as u64, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: unknown `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	// Proof Skipped: unknown `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	fn apply_item_actions_with_signature(s: u32, ) -> Weight {
		Weight::from_parts(162_349_356 as u64, 0)
			// Standard Error: 436
			.saturating_add(Weight::from_parts(13_673 as u64, 0).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	// Proof Skipped: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn upsert_page_with_signature(s: u32, ) -> Weight {
		Weight::from_parts(87_513_351 as u64, 0)
			// Standard Error: 394
			.saturating_add(Weight::from_parts(5_610 as u64, 0).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Schemas Schemas (r:1 w:0)
	// Proof Skipped: Schemas Schemas (max_values: None, max_size: None, mode: Measured)
	// Storage: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	// Proof Skipped: unknown `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page_with_signature() -> Weight {
		Weight::from_parts(90_185_000 as u64, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
