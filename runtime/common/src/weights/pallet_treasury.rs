//! Autogenerated weights for pallet_treasury
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_treasury
// --extrinsic
// *
// --chain=frequency-bench
// --execution=wasm
// --heap-pages=4096
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --output=./scripts/../runtime/common/src/weights
// --template=./scripts/../.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_treasury using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for SubstrateWeight<T> {
	fn spend() -> Weight {
		Weight::from_parts(286_000 as u64, 0)
	}
	// Storage: Treasury ProposalCount (r:1 w:1)
	// Proof: Treasury ProposalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Treasury Proposals (r:0 w:1)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	fn propose_spend() -> Weight {
		Weight::from_parts(27_969_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Treasury Proposals (r:1 w:1)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn reject_proposal() -> Weight {
		Weight::from_parts(33_944_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Treasury Proposals (r:1 w:0)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(258), added: 753, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 63]`.
	fn approve_proposal(p: u32, ) -> Weight {
		Weight::from_parts(10_465_810 as u64, 0)
			// Standard Error: 1_278
			.saturating_add(Weight::from_parts(107_313 as u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(258), added: 753, mode: MaxEncodedLen)
	fn remove_approval() -> Weight {
		Weight::from_parts(6_896_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: System Account (r:129 w:129)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Treasury Deactivated (r:1 w:1)
	// Proof: Treasury Deactivated (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(258), added: 753, mode: MaxEncodedLen)
	// Storage: Treasury Proposals (r:64 w:64)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 64]`.
	fn on_initialize_proposals(p: u32, ) -> Weight {
		Weight::from_parts(43_013_858 as u64, 0)
			// Standard Error: 30_366
			.saturating_add(Weight::from_parts(43_118_103 as u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(p as u64)))
	}
}
