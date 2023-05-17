//! Autogenerated weights for pallet_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet
// pallet_collator_selection
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
// --output=./scripts/../runtime/common/src/weights/pallet_collator_selection.rs
// --template=./scripts/../.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_collator_selection using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for SubstrateWeight<T> {
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// The range of component `b` is `[1, 16]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		Weight::from_ref_time(17_740_791 as u64)
			// Standard Error: 8_858
			.saturating_add(Weight::from_ref_time(2_798_811 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	fn set_desired_candidates() -> Weight {
		Weight::from_ref_time(14_806_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	fn set_candidacy_bond() -> Weight {
		Weight::from_ref_time(12_782_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// The range of component `c` is `[1, 49]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		Weight::from_ref_time(44_949_353 as u64)
			// Standard Error: 3_558
			.saturating_add(Weight::from_ref_time(408_175 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// The range of component `c` is `[2, 50]`.
	fn leave_intent(c: u32, ) -> Weight {
		Weight::from_ref_time(31_613_931 as u64)
			// Standard Error: 3_069
			.saturating_add(Weight::from_ref_time(324_561 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	fn note_author() -> Weight {
		Weight::from_ref_time(32_159_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection LastAuthoredBlock (r:50 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// The range of component `r` is `[1, 50]`.
	/// The range of component `c` is `[1, 50]`.
	fn new_session(_r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(17_936_000 as u64)
			// Standard Error: 248_616
			.saturating_add(Weight::from_ref_time(9_940_553 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}
