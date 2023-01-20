//! Autogenerated weights for orml_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/production/frequency
// benchmark
// pallet
// --pallet
// orml_vesting
// --extrinsic
// *
// --chain=frequency-bench
// --execution
// wasm
// --heap-pages=4096
// --wasm-execution
// compiled
// --steps=50
// --repeat=20
// --output=./scripts/../runtime/common/src/weights/orml_vesting.rs
// --template=./scripts/../.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for orml_vesting using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_vesting::WeightInfo for SubstrateWeight<T> {
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	fn vested_transfer() -> Weight {
		Weight::from_ref_time(62_431_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `i` is `[1, 50]`.
	fn claim(i: u32, ) -> Weight {
		Weight::from_ref_time(40_847_535 as u64)
			// Standard Error: 1_718
			.saturating_add(Weight::from_ref_time(77_915 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Vesting VestingSchedules (r:0 w:1)
	/// The range of component `i` is `[1, 50]`.
	fn update_vesting_schedules(i: u32, ) -> Weight {
		Weight::from_ref_time(34_323_520 as u64)
			// Standard Error: 1_293
			.saturating_add(Weight::from_ref_time(58_482 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
