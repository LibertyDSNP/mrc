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

//! Autogenerated weights for pallet_messages
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-31, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("./scripts/../res/genesis/testnet/frequency-spec-rococo-testnet.json"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet
// pallet_messages
// --extrinsic
// *
// --chain=./scripts/../res/genesis/testnet/frequency-spec-rococo-testnet.json
// --execution
// wasm
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 10
// --output=./scripts/../pallets/messages/src/weights.rs
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

/// Weight functions needed for pallet_messages.
pub trait WeightInfo {
	fn add_onchain_message(n: u32, m: u32, ) -> Weight;
	fn add_ipfs_message(n: u32, m: u32, ) -> Weight;
	fn on_initialize(m: u32, s: u32, ) -> Weight;
}

/// Weights for pallet_messages using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Messages BlockMessages (r:1 w:1)
	fn add_onchain_message(n: u32, m: u32, ) -> Weight {
		(26_795_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 3_000
			.saturating_add((383_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Messages BlockMessages (r:1 w:1)
	fn add_ipfs_message(n: u32, m: u32, ) -> Weight {
		(24_901_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 2_000
			.saturating_add((334_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Messages BlockMessages (r:1 w:1)
	// Storage: Messages Messages (r:0 w:1)
	fn on_initialize(m: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((734_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 172_000
			.saturating_add((10_296_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Messages BlockMessages (r:1 w:1)
	fn add_onchain_message(n: u32, m: u32, ) -> Weight {
		(26_795_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 3_000
			.saturating_add((383_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Messages BlockMessages (r:1 w:1)
	fn add_ipfs_message(n: u32, m: u32, ) -> Weight {
		(24_901_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 2_000
			.saturating_add((334_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Messages BlockMessages (r:1 w:1)
	// Storage: Messages Messages (r:0 w:1)
	fn on_initialize(m: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((734_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 172_000
			.saturating_add((10_296_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
}
