//! Autogenerated weights for pallet_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-12-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-4-74`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_collator_selection
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --output=./scripts/../runtime/common/src/weights
// --template=./scripts/../.maintain/runtime-weight-template.hbs
// --additional-trie-layers=3

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weights for `pallet_collator_selection` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for SubstrateWeight<T> {
	/// Storage: `Session::NextKeys` (r:16 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::Invulnerables` (r:0 w:1)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(513), added: 1008, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 16]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `334 + b * (79 ±0)`
		//  Estimated: `1819 + b * (2554 ±0)`
		// Minimum execution time: 10_351_000 picoseconds.
		Weight::from_parts(8_808_029, 1819)
			// Standard Error: 9_223
			.saturating_add(Weight::from_parts(3_119_009, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2554).saturating_mul(b.into()))
	}
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:1)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(513), added: 1008, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(2401), added: 2896, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 15]`.
	/// The range of component `c` is `[1, 49]`.
	fn add_invulnerable(b: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111 + b * (32 ±0) + c * (52 ±0)`
		//  Estimated: `5025 + b * (29 ±0) + c * (53 ±0)`
		// Minimum execution time: 40_226_000 picoseconds.
		Weight::from_parts(41_059_784, 5025)
			// Standard Error: 4_364
			.saturating_add(Weight::from_parts(19_135, 0).saturating_mul(b.into()))
			// Standard Error: 1_294
			.saturating_add(Weight::from_parts(172_404, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 29).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 53).saturating_mul(c.into()))
	}
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(2401), added: 2896, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:1)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(513), added: 1008, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[2, 16]`.
	fn remove_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153 + b * (32 ±0)`
		//  Estimated: `4381`
		// Minimum execution time: 9_535_000 picoseconds.
		Weight::from_parts(9_669_175, 4381)
			// Standard Error: 2_127
			.saturating_add(Weight::from_parts(167_891, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `CollatorSelection::DesiredCandidates` (r:0 w:1)
	/// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_desired_candidates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_756_000 picoseconds.
		Weight::from_parts(4_018_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `CollatorSelection::CandidacyBond` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(2401), added: 2896, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:50 w:50)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:50)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 50]`.
	/// The range of component `k` is `[0, 50]`.
	fn set_candidacy_bond(c: u32, k: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (190 ±0) + k * (131 ±0)`
		//  Estimated: `4381 + c * (894 ±29) + k * (894 ±29)`
		// Minimum execution time: 8_153_000 picoseconds.
		Weight::from_parts(8_334_000, 4381)
			// Standard Error: 152_200
			.saturating_add(Weight::from_parts(5_281_343, 0).saturating_mul(c.into()))
			// Standard Error: 152_200
			.saturating_add(Weight::from_parts(4_964_871, 0).saturating_mul(k.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k.into())))
			.saturating_add(Weight::from_parts(0, 894).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 894).saturating_mul(k.into()))
	}
	/// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(2401), added: 2896, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 50]`.
	fn update_bond(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `372 + c * (49 ±0)`
		//  Estimated: `4381`
		// Minimum execution time: 22_641_000 picoseconds.
		Weight::from_parts(25_012_851, 4381)
			// Standard Error: 2_648
			.saturating_add(Weight::from_parts(189_363, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(2401), added: 2896, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(513), added: 1008, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 49]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `810 + c * (57 ±0)`
		//  Estimated: `4774 + c * (57 ±0)`
		// Minimum execution time: 34_009_000 picoseconds.
		Weight::from_parts(36_559_424, 4774)
			// Standard Error: 3_558
			.saturating_add(Weight::from_parts(254_031, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 57).saturating_mul(c.into()))
	}
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(513), added: 1008, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(2401), added: 2896, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:2)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 50]`.
	fn take_candidate_slot(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `901 + c * (58 ±0)`
		//  Estimated: `4866 + c * (59 ±0)`
		// Minimum execution time: 48_246_000 picoseconds.
		Weight::from_parts(51_626_277, 4866)
			// Standard Error: 3_859
			.saturating_add(Weight::from_parts(280_166, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 59).saturating_mul(c.into()))
	}
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(2401), added: 2896, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(513), added: 1008, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 50]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `501 + c * (48 ±0)`
		//  Estimated: `4381`
		// Minimum execution time: 25_617_000 picoseconds.
		Weight::from_parts(27_539_028, 4381)
			// Standard Error: 2_763
			.saturating_add(Weight::from_parts(226_454, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `212`
		//  Estimated: `6691`
		// Minimum execution time: 37_950_000 picoseconds.
		Weight::from_parts(38_940_000, 6691)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(2401), added: 2896, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:50 w:0)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(513), added: 1008, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::DesiredCandidates` (r:1 w:0)
	/// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:49 w:49)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 50]`.
	/// The range of component `c` is `[1, 50]`.
	fn new_session(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1172 + c * (98 ±0) + r * (130 ±0)`
		//  Estimated: `4381 + c * (2519 ±0) + r * (2603 ±0)`
		// Minimum execution time: 16_795_000 picoseconds.
		Weight::from_parts(17_113_000, 4381)
			// Standard Error: 269_245
			.saturating_add(Weight::from_parts(11_962_358, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2519).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(r.into()))
	}
}


#[cfg(test)]
mod tests {
  use frame_support::{traits::Get, weights::Weight, dispatch::DispatchClass};
  use crate::constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
  use crate::weights::extrinsic_weights::ExtrinsicBaseWeight;

  struct BlockWeights;
  impl Get<frame_system::limits::BlockWeights> for BlockWeights {
  	fn get() -> frame_system::limits::BlockWeights {
  		frame_system::limits::BlockWeights::builder()
  			.base_block(Weight::zero())
  			.for_class(DispatchClass::all(), |weights| {
  				weights.base_extrinsic = ExtrinsicBaseWeight::get().into();
  			})
  			.for_class(DispatchClass::non_mandatory(), |weights| {
  				weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
  			})
  			.build_or_panic()
  	}
  }

	#[test]
	fn test_set_invulnerables() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 1819
		);
	}
	#[test]
	fn test_add_invulnerable() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5025
		);
	}
	#[test]
	fn test_remove_invulnerable() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4381
		);
	}
	#[test]
	fn test_set_candidacy_bond() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4381
		);
	}
	#[test]
	fn test_update_bond() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4381
		);
	}
	#[test]
	fn test_register_as_candidate() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4774
		);
	}
	#[test]
	fn test_take_candidate_slot() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4866
		);
	}
	#[test]
	fn test_leave_intent() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4381
		);
	}
	#[test]
	fn test_note_author() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6691
		);
	}
	#[test]
	fn test_new_session() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4381
		);
	}
}
