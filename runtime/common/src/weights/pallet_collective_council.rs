//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-12, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-11-249`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_collective
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --additional-trie-layers=5
// --steps=50
// --repeat=20
// --output=./scripts/../runtime/common/src/weights
// --template=./scripts/../.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weights for `pallet_collective` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for SubstrateWeight<T> {
	/// Storage: `Council::Members` (r:1 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:25 w:25)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 10]`.
	/// The range of component `n` is `[0, 10]`.
	/// The range of component `p` is `[0, 25]`.
	/// The range of component `m` is `[0, 10]`.
	/// The range of component `n` is `[0, 10]`.
	/// The range of component `p` is `[0, 25]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (832 ±0) + p * (310 ±0)`
		//  Estimated: `5779 + m * (489 ±3) + p * (2615 ±1)`
		// Minimum execution time: 7_123_000 picoseconds.
		Weight::from_parts(7_514_000, 5779)
			// Standard Error: 62_392
			.saturating_add(Weight::from_parts(2_942_687, 0).saturating_mul(m.into()))
			// Standard Error: 25_354
			.saturating_add(Weight::from_parts(3_367_978, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 489).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 2615).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70 + m * (32 ±0)`
		//  Estimated: `3039 + m * (32 ±0)`
		// Minimum execution time: 10_958_000 picoseconds.
		Weight::from_parts(11_278_269, 3039)
			// Standard Error: 14
			.saturating_add(Weight::from_parts(1_468, 0).saturating_mul(b.into()))
			// Standard Error: 1_514
			.saturating_add(Weight::from_parts(24_855, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:0)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70 + m * (32 ±0)`
		//  Estimated: `5019 + m * (32 ±0)`
		// Minimum execution time: 13_457_000 picoseconds.
		Weight::from_parts(13_530_036, 5019)
			// Standard Error: 15
			.saturating_add(Weight::from_parts(1_566, 0).saturating_mul(b.into()))
			// Standard Error: 1_706
			.saturating_add(Weight::from_parts(41_320, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113 + m * (32 ±0) + p * (51 ±0)`
		//  Estimated: `5015 + m * (42 ±0) + p * (49 ±0)`
		// Minimum execution time: 18_291_000 picoseconds.
		Weight::from_parts(17_181_113, 5015)
			// Standard Error: 43
			.saturating_add(Weight::from_parts(2_270, 0).saturating_mul(b.into()))
			// Standard Error: 5_180
			.saturating_add(Weight::from_parts(154_925, 0).saturating_mul(m.into()))
			// Standard Error: 1_807
			.saturating_add(Weight::from_parts(260_770, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 42).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 10]`.
	/// The range of component `m` is `[5, 10]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `571 + m * (64 ±0)`
		//  Estimated: `5521 + m * (64 ±0)`
		// Minimum execution time: 16_466_000 picoseconds.
		Weight::from_parts(17_352_913, 5521)
			// Standard Error: 4_520
			.saturating_add(Weight::from_parts(45_710, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41 + m * (64 ±0) + p * (54 ±0)`
		//  Estimated: `5009 + m * (80 ±0) + p * (50 ±0)`
		// Minimum execution time: 18_992_000 picoseconds.
		Weight::from_parts(18_873_473, 5009)
			// Standard Error: 5_748
			.saturating_add(Weight::from_parts(141_312, 0).saturating_mul(m.into()))
			// Standard Error: 1_533
			.saturating_add(Weight::from_parts(244_436, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 50).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `72 + b * (1 ±0) + m * (64 ±0) + p * (72 ±0)`
		//  Estimated: `5015 + b * (1 ±0) + m * (85 ±0) + p * (65 ±0)`
		// Minimum execution time: 28_153_000 picoseconds.
		Weight::from_parts(28_549_556, 5015)
			// Standard Error: 62
			.saturating_add(Weight::from_parts(1_927, 0).saturating_mul(b.into()))
			// Standard Error: 9_672
			.saturating_add(Weight::from_parts(105_503, 0).saturating_mul(m.into()))
			// Standard Error: 2_587
			.saturating_add(Weight::from_parts(401_294, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 85).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `61 + m * (64 ±0) + p * (54 ±0)`
		//  Estimated: `5029 + m * (80 ±0) + p * (50 ±0)`
		// Minimum execution time: 20_468_000 picoseconds.
		Weight::from_parts(20_415_435, 5029)
			// Standard Error: 6_214
			.saturating_add(Weight::from_parts(169_915, 0).saturating_mul(m.into()))
			// Standard Error: 1_657
			.saturating_add(Weight::from_parts(246_463, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 50).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `92 + b * (1 ±0) + m * (64 ±0) + p * (72 ±0)`
		//  Estimated: `5035 + b * (1 ±0) + m * (85 ±0) + p * (65 ±0)`
		// Minimum execution time: 29_816_000 picoseconds.
		Weight::from_parts(30_873_954, 5035)
			// Standard Error: 65
			.saturating_add(Weight::from_parts(1_999, 0).saturating_mul(b.into()))
			// Standard Error: 10_140
			.saturating_add(Weight::from_parts(37_551, 0).saturating_mul(m.into()))
			// Standard Error: 2_712
			.saturating_add(Weight::from_parts(425_069, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 85).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(p.into()))
	}
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `p` is `[1, 25]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226 + p * (32 ±0)`
		//  Estimated: `3196 + p * (32 ±0)`
		// Minimum execution time: 11_587_000 picoseconds.
		Weight::from_parts(12_279_358, 3196)
			// Standard Error: 952
			.saturating_add(Weight::from_parts(202_031, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
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
	fn test_set_members() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5779
		);
	}
	#[test]
	fn test_execute() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3039
		);
	}
	#[test]
	fn test_propose_execute() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5019
		);
	}
	#[test]
	fn test_propose_proposed() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5015
		);
	}
	#[test]
	fn test_vote() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5521
		);
	}
	#[test]
	fn test_close_early_disapproved() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5009
		);
	}
	#[test]
	fn test_close_early_approved() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5015
		);
	}
	#[test]
	fn test_close_disapproved() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5029
		);
	}
	#[test]
	fn test_close_approved() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5035
		);
	}
	#[test]
	fn test_disapprove_proposal() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3196
		);
	}
}
