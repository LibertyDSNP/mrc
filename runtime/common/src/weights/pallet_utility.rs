//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-10-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-4-234`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_utility
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

/// Weights for `pallet_utility` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for SubstrateWeight<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_659_000 picoseconds.
		Weight::from_parts(4_684_142, 0)
			// Standard Error: 1_204
			.saturating_add(Weight::from_parts(2_644_145, 0).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_808_000 picoseconds.
		Weight::from_parts(3_996_000, 0)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_534_000 picoseconds.
		Weight::from_parts(6_328_579, 0)
			// Standard Error: 1_440
			.saturating_add(Weight::from_parts(2_819_825, 0).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_561_000 picoseconds.
		Weight::from_parts(5_870_000, 0)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_568_000 picoseconds.
		Weight::from_parts(3_155_056, 0)
			// Standard Error: 1_586
			.saturating_add(Weight::from_parts(2_638_199, 0).saturating_mul(c.into()))
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

}
