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

//! Autogenerated weights for pallet_msa
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-19, STEPS: `5`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/production/frequency
// benchmark
// pallet
// --pallet
// pallet_msa
// --extrinsic
// *
// --chain=frequency
// --execution
// wasm
// --wasm-execution
// compiled
// --steps
// 5
// --repeat
// 5
// --output=./scripts/../pallets/msa/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(
rustdoc::all,
missing_docs,
unused_parens,
unused_imports
)]

use frame_support::{traits::Get, weights::{constants::RocksDbWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_msa.
pub trait WeightInfo {
	fn create(s: u32) -> Weight;
	fn create_sponsored_account_with_delegation() -> Weight;
	fn revoke_delegation_by_provider(s: u32) -> Weight;
	fn add_key_to_msa() -> Weight;
	fn delete_msa_public_key() -> Weight;
	fn retire_msa() -> Weight;
	fn grant_delegation() -> Weight;
	fn revoke_delegation_by_delegator() -> Weight;
	fn register_provider() -> Weight;
	fn on_initialize(s: u32) -> Weight;
}

/// Weights for pallet_msa using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn create(s: u32, ) -> Weight {
		Weight::from_ref_time(17_462_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(11_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn create_sponsored_account_with_delegation() -> Weight {
		Weight::from_ref_time(61_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn revoke_delegation_by_provider(s: u32) -> Weight {
		Weight::from_ref_time(20_484_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(22_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn add_key_to_msa() -> Weight {
		Weight::from_ref_time(55_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn delete_msa_public_key() -> Weight {
		Weight::from_ref_time(18_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn retire_msa() -> Weight {
		Weight::from_ref_time(20_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:0)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn grant_delegation() -> Weight {
		Weight::from_ref_time(59_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn revoke_delegation_by_delegator() -> Weight {
		Weight::from_ref_time(15_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:1)
	fn register_provider() -> Weight {
		Weight::from_ref_time(13_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn on_initialize(s: u32) -> Weight {
		Weight::from_ref_time(15_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(s as u64))
			.saturating_mul(2 as u64)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn create(s: u32, ) -> Weight {
		Weight::from_ref_time(17_462_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(11_000 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn create_sponsored_account_with_delegation() -> Weight {
		Weight::from_ref_time(61_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn revoke_delegation_by_provider(s: u32, ) -> Weight {
		Weight::from_ref_time(18_059_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(15_000 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn add_key_to_msa() -> Weight {
		Weight::from_ref_time(55_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn delete_msa_public_key() -> Weight {
		Weight::from_ref_time(18_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn retire_msa() -> Weight {
		Weight::from_ref_time(20_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:0)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn grant_delegation() -> Weight {
		Weight::from_ref_time(59_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn revoke_delegation_by_delegator() -> Weight {
		Weight::from_ref_time(15_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:1)
	fn register_provider() -> Weight {
		Weight::from_ref_time(13_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}

	fn on_initialize(s: u32) -> Weight {
		Weight::from_ref_time(15_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(s as u64))
			.saturating_mul(2 as u64)
	}
}
