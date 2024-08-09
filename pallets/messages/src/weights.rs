
//! Autogenerated weights for `pallet_messages`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-09, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-11-214`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("frequency-bench")`, DB CACHE: `1024`

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_messages
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/messages/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs
// --additional-trie-layers=3

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_messages`.
pub trait WeightInfo {
	fn add_onchain_message(n: u32, ) -> Weight;
	fn add_ipfs_message() -> Weight;
}

/// Weights for `pallet_messages` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Messages::MessagesV2` (r:0 w:1)
	/// Proof: `Messages::MessagesV2` (`max_values`: None, `max_size`: Some(3123), added: 5598, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 3071]`.
	fn add_onchain_message(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329`
		//  Estimated: `4177`
		// Minimum execution time: 29_199_000 picoseconds.
		Weight::from_parts(33_942_784, 4177)
			// Standard Error: 356
			.saturating_add(Weight::from_parts(694, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Messages::MessagesV2` (r:0 w:1)
	/// Proof: `Messages::MessagesV2` (`max_values`: None, `max_size`: Some(3123), added: 5598, mode: `MaxEncodedLen`)
	fn add_ipfs_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `722`
		//  Estimated: `4008`
		// Minimum execution time: 29_651_000 picoseconds.
		Weight::from_parts(30_236_000, 4008)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Messages::MessagesV2` (r:0 w:1)
	/// Proof: `Messages::MessagesV2` (`max_values`: None, `max_size`: Some(3123), added: 5598, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 3071]`.
	fn add_onchain_message(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329`
		//  Estimated: `4177`
		// Minimum execution time: 29_199_000 picoseconds.
		Weight::from_parts(33_942_784, 4177)
			// Standard Error: 356
			.saturating_add(Weight::from_parts(694, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Messages::MessagesV2` (r:0 w:1)
	/// Proof: `Messages::MessagesV2` (`max_values`: None, `max_size`: Some(3123), added: 5598, mode: `MaxEncodedLen`)
	fn add_ipfs_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `722`
		//  Estimated: `4008`
		// Minimum execution time: 29_651_000 picoseconds.
		Weight::from_parts(30_236_000, 4008)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}


#[cfg(test)]
mod tests {
  use frame_support::{traits::Get, weights::Weight, dispatch::DispatchClass};
  use common_runtime::constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
  use common_runtime::weights::extrinsic_weights::ExtrinsicBaseWeight;

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
	fn test_add_onchain_message() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4177
		);
	}
	#[test]
	fn test_add_ipfs_message() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4008
		);
	}
}
