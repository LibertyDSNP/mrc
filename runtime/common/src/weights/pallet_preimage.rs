//! Autogenerated weights for pallet_preimage
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_preimage
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

/// Weights for pallet_preimage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::WeightInfo for SubstrateWeight<T> {
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	// Storage: Preimage PreimageFor (r:0 w:1)
	// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_preimage(s: u32, ) -> Weight {
		Weight::from_parts(30_556_000 as u64, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(2_284 as u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	// Storage: Preimage PreimageFor (r:0 w:1)
	// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_requested_preimage(s: u32, ) -> Weight {
		Weight::from_parts(15_697_000 as u64, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(2_265 as u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	// Storage: Preimage PreimageFor (r:0 w:1)
	// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		Weight::from_parts(14_977_000 as u64, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(2_284 as u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	// Storage: Preimage PreimageFor (r:0 w:1)
	// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	fn unnote_preimage() -> Weight {
		Weight::from_parts(47_072_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	// Storage: Preimage PreimageFor (r:0 w:1)
	// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	fn unnote_no_deposit_preimage() -> Weight {
		Weight::from_parts(30_957_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn request_preimage() -> Weight {
		Weight::from_parts(25_559_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn request_no_deposit_preimage() -> Weight {
		Weight::from_parts(14_747_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn request_unnoted_preimage() -> Weight {
		Weight::from_parts(15_305_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn request_requested_preimage() -> Weight {
		Weight::from_parts(11_272_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	// Storage: Preimage PreimageFor (r:0 w:1)
	// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
	fn unrequest_preimage() -> Weight {
		Weight::from_parts(31_139_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn unrequest_unnoted_preimage() -> Weight {
		Weight::from_parts(8_791_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn unrequest_multi_referenced_preimage() -> Weight {
		Weight::from_parts(9_120_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
