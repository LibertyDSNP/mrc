//! Autogenerated weights for pallet_treasury
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-19, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/production/frequency
// benchmark
// pallet
// --pallet
// pallet_treasury
// --extrinsic
// *
// --chain=frequency
// --execution
// wasm
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 10
// --output=./scripts/../runtime/common/src/weights/pallet_treasury.rs
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
		Weight::from_ref_time(0 as u64)
	}
	// Storage: Treasury ProposalCount (r:1 w:1)
	// Storage: Treasury Proposals (r:0 w:1)
	fn propose_spend() -> Weight {
		Weight::from_ref_time(23_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Treasury Proposals (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reject_proposal() -> Weight {
		Weight::from_ref_time(29_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Treasury Proposals (r:1 w:0)
	// Storage: Treasury Approvals (r:1 w:1)
	/// The range of component `p` is `[0, 63]`.
	fn approve_proposal(p: u32, ) -> Weight {
		Weight::from_ref_time(10_740_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(37_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Treasury Approvals (r:1 w:1)
	fn remove_approval() -> Weight {
		Weight::from_ref_time(7_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: System Account (r:1 w:0)
	// Storage: Treasury Approvals (r:1 w:1)
	// Storage: Treasury Proposals (r:1 w:1)
	/// The range of component `p` is `[0, 64]`.
	fn on_initialize_proposals(p: u32, ) -> Weight {
		Weight::from_ref_time(23_915_000 as u64)
			// Standard Error: 87_000
			.saturating_add(Weight::from_ref_time(24_286_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(p as u64)))
	}
}
