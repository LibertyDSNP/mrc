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

//! Autogenerated weights for pallet_schemas
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-07, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-runner-qpqf8-fp5d5`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_schemas
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --additional-trie-layers=20
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/schemas/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_schemas.
pub trait WeightInfo {
	fn create_schema(m: u32, ) -> Weight;
	fn create_schema_via_governance(m: u32, ) -> Weight;
	fn propose_to_create_schema(m: u32, ) -> Weight;
	fn set_max_schema_model_bytes() -> Weight;
}

/// Weights for pallet_schemas using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Schemas GovernanceSchemaModelMaxBytes (r:1 w:0)
	/// Proof: Schemas GovernanceSchemaModelMaxBytes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:1)
	/// Proof: Schemas CurrentSchemaIdentifierMaximum (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: Schemas Schemas (r:0 w:1)
	/// Proof: Schemas Schemas (max_values: None, max_size: Some(65518), added: 67993, mode: MaxEncodedLen)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `10399`
		// Minimum execution time: 14_864_000 picoseconds.
		Weight::from_parts(15_061_000, 10399)
			// Standard Error: 60
			.saturating_add(Weight::from_parts(39_004, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Schemas GovernanceSchemaModelMaxBytes (r:1 w:0)
	/// Proof: Schemas GovernanceSchemaModelMaxBytes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:1)
	/// Proof: Schemas CurrentSchemaIdentifierMaximum (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: Schemas Schemas (r:0 w:1)
	/// Proof: Schemas Schemas (max_values: None, max_size: Some(65518), added: 67993, mode: MaxEncodedLen)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_via_governance(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `10399`
		// Minimum execution time: 14_778_000 picoseconds.
		Weight::from_parts(14_885_000, 10399)
			// Standard Error: 61
			.saturating_add(Weight::from_parts(38_923, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalCount (r:1 w:1)
	/// Proof Skipped: Council ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[16, 65499]`.
	fn propose_to_create_schema(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `230`
		//  Estimated: `12605`
		// Minimum execution time: 22_909_000 picoseconds.
		Weight::from_parts(16_340_405, 12605)
			// Standard Error: 51
			.saturating_add(Weight::from_parts(3_863, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Schemas GovernanceSchemaModelMaxBytes (r:0 w:1)
	/// Proof: Schemas GovernanceSchemaModelMaxBytes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_max_schema_model_bytes() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_645_000 picoseconds.
		Weight::from_parts(7_083_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Schemas GovernanceSchemaModelMaxBytes (r:1 w:0)
	/// Proof: Schemas GovernanceSchemaModelMaxBytes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:1)
	/// Proof: Schemas CurrentSchemaIdentifierMaximum (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: Schemas Schemas (r:0 w:1)
	/// Proof: Schemas Schemas (max_values: None, max_size: Some(65518), added: 67993, mode: MaxEncodedLen)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `10399`
		// Minimum execution time: 14_864_000 picoseconds.
		Weight::from_parts(15_061_000, 10399)
			// Standard Error: 60
			.saturating_add(Weight::from_parts(39_004, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Schemas GovernanceSchemaModelMaxBytes (r:1 w:0)
	/// Proof: Schemas GovernanceSchemaModelMaxBytes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:1)
	/// Proof: Schemas CurrentSchemaIdentifierMaximum (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: Schemas Schemas (r:0 w:1)
	/// Proof: Schemas Schemas (max_values: None, max_size: Some(65518), added: 67993, mode: MaxEncodedLen)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_via_governance(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `10399`
		// Minimum execution time: 14_778_000 picoseconds.
		Weight::from_parts(14_885_000, 10399)
			// Standard Error: 61
			.saturating_add(Weight::from_parts(38_923, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalCount (r:1 w:1)
	/// Proof Skipped: Council ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[16, 65499]`.
	fn propose_to_create_schema(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `230`
		//  Estimated: `12605`
		// Minimum execution time: 22_909_000 picoseconds.
		Weight::from_parts(16_340_405, 12605)
			// Standard Error: 51
			.saturating_add(Weight::from_parts(3_863, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Schemas GovernanceSchemaModelMaxBytes (r:0 w:1)
	/// Proof: Schemas GovernanceSchemaModelMaxBytes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_max_schema_model_bytes() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_645_000 picoseconds.
		Weight::from_parts(7_083_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
