
//! Autogenerated weights for `pallet_msa`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-21, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-4-165`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("frequency-bench")`, DB CACHE: `1024`

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_msa
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/msa/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs
// --additional-trie-layers=3

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_msa`.
pub trait WeightInfo {
	fn create() -> Weight;
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight;
	fn revoke_delegation_by_provider() -> Weight;
	fn add_public_key_to_msa() -> Weight;
	fn delete_msa_public_key() -> Weight;
	fn retire_msa() -> Weight;
	fn grant_delegation(s: u32, ) -> Weight;
	fn revoke_delegation_by_delegator() -> Weight;
	fn create_provider() -> Weight;
	fn create_provider_via_governance() -> Weight;
	fn propose_to_be_provider() -> Weight;
	fn revoke_schema_permissions(s: u32, ) -> Weight;
}

/// Weights for `pallet_msa` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Msa::CurrentMsaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Msa::CurrentMsaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `4008`
		// Minimum execution time: 11_703_000 picoseconds.
		Weight::from_parts(12_559_000, 4008)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:2 w:2)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Msa::CurrentMsaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Msa::CurrentMsaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1284`
		//  Estimated: `6531`
		// Minimum execution time: 97_954_000 picoseconds.
		Weight::from_parts(101_792_980, 6531)
			// Standard Error: 13_128
			.saturating_add(Weight::from_parts(93_890, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	fn revoke_delegation_by_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `4177`
		// Minimum execution time: 13_595_000 picoseconds.
		Weight::from_parts(14_115_000, 4177)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:4 w:4)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn add_public_key_to_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1574`
		//  Estimated: `9981`
		// Minimum execution time: 140_973_000 picoseconds.
		Weight::from_parts(144_738_000, 9981)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn delete_msa_public_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `254`
		//  Estimated: `6531`
		// Minimum execution time: 24_558_000 picoseconds.
		Weight::from_parts(25_585_000, 6531)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn retire_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `71`
		//  Estimated: `4008`
		// Minimum execution time: 17_596_000 picoseconds.
		Weight::from_parts(18_526_000, 4008)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:2 w:2)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn grant_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1327`
		//  Estimated: `6531`
		// Minimum execution time: 89_510_000 picoseconds.
		Weight::from_parts(91_652_737, 6531)
			// Standard Error: 9_268
			.saturating_add(Weight::from_parts(138_805, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	fn revoke_delegation_by_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `4177`
		// Minimum execution time: 13_631_000 picoseconds.
		Weight::from_parts(14_201_000, 4177)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:1)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	fn create_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `4008`
		// Minimum execution time: 10_895_000 picoseconds.
		Weight::from_parts(11_486_000, 4008)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:1)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	fn create_provider_via_governance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `4008`
		// Minimum execution time: 10_582_000 picoseconds.
		Weight::from_parts(11_040_000, 4008)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn propose_to_be_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `4107`
		// Minimum execution time: 18_807_000 picoseconds.
		Weight::from_parts(19_452_000, 4107)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn revoke_schema_permissions(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316 + s * (6 ±0)`
		//  Estimated: `4177`
		// Minimum execution time: 16_833_000 picoseconds.
		Weight::from_parts(17_738_522, 4177)
			// Standard Error: 4_179
			.saturating_add(Weight::from_parts(117_796, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Msa::CurrentMsaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Msa::CurrentMsaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `4008`
		// Minimum execution time: 11_703_000 picoseconds.
		Weight::from_parts(12_559_000, 4008)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:2 w:2)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Msa::CurrentMsaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Msa::CurrentMsaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1284`
		//  Estimated: `6531`
		// Minimum execution time: 97_954_000 picoseconds.
		Weight::from_parts(101_792_980, 6531)
			// Standard Error: 13_128
			.saturating_add(Weight::from_parts(93_890, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	fn revoke_delegation_by_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `4177`
		// Minimum execution time: 13_595_000 picoseconds.
		Weight::from_parts(14_115_000, 4177)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:4 w:4)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn add_public_key_to_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1574`
		//  Estimated: `9981`
		// Minimum execution time: 140_973_000 picoseconds.
		Weight::from_parts(144_738_000, 9981)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn delete_msa_public_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `254`
		//  Estimated: `6531`
		// Minimum execution time: 24_558_000 picoseconds.
		Weight::from_parts(25_585_000, 6531)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn retire_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `71`
		//  Estimated: `4008`
		// Minimum execution time: 17_596_000 picoseconds.
		Weight::from_parts(18_526_000, 4008)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:2 w:2)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn grant_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1327`
		//  Estimated: `6531`
		// Minimum execution time: 89_510_000 picoseconds.
		Weight::from_parts(91_652_737, 6531)
			// Standard Error: 9_268
			.saturating_add(Weight::from_parts(138_805, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	fn revoke_delegation_by_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `4177`
		// Minimum execution time: 13_631_000 picoseconds.
		Weight::from_parts(14_201_000, 4177)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:1)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	fn create_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `4008`
		// Minimum execution time: 10_895_000 picoseconds.
		Weight::from_parts(11_486_000, 4008)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:1)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	fn create_provider_via_governance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `4008`
		// Minimum execution time: 10_582_000 picoseconds.
		Weight::from_parts(11_040_000, 4008)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn propose_to_be_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `4107`
		// Minimum execution time: 18_807_000 picoseconds.
		Weight::from_parts(19_452_000, 4107)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn revoke_schema_permissions(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316 + s * (6 ±0)`
		//  Estimated: `4177`
		// Minimum execution time: 16_833_000 picoseconds.
		Weight::from_parts(17_738_522, 4177)
			// Standard Error: 4_179
			.saturating_add(Weight::from_parts(117_796, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}


#[cfg(test)]
mod tests {
  use frame_support::{traits::Get, weights::Weight, dispatch::DispatchClass};
  use common_runtime::constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
  use common_runtime::weights::extrinsic_weights::ExtrinsicBaseWeight;

  #[allow(dead_code)]
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
	fn test_create() {
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
	#[test]
	fn test_create_sponsored_account_with_delegation() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6531
		);
	}
	#[test]
	fn test_revoke_delegation_by_provider() {
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
	fn test_add_public_key_to_msa() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 9981
		);
	}
	#[test]
	fn test_delete_msa_public_key() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6531
		);
	}
	#[test]
	fn test_retire_msa() {
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
	#[test]
	fn test_grant_delegation() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6531
		);
	}
	#[test]
	fn test_revoke_delegation_by_delegator() {
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
	fn test_create_provider() {
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
	#[test]
	fn test_create_provider_via_governance() {
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
	#[test]
	fn test_propose_to_be_provider() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4107
		);
	}
	#[test]
	fn test_revoke_schema_permissions() {
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
}
