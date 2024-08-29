#![allow(missing_docs)]
use common_primitives::node::AccountId;
use common_runtime::constants::{
	currency::EXISTENTIAL_DEPOSIT, FREQUENCY_LOCAL_TOKEN, TOKEN_DECIMALS,
};
use cumulus_primitives_core::ParaId;
use frequency_runtime::{AuraId, CouncilConfig, Ss58Prefix, SudoConfig, TechnicalCommitteeConfig};
use polkadot_service::chain_spec::Extensions as RelayChainExtensions;
use sc_service::ChainType;
use sp_runtime::traits::AccountIdConversion;

use super::{get_account_id_from_seed, get_collator_keys_from_seed, get_properties, Extensions};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;
use sp_core::sr25519;

// Generic chain spec, in case when we don't have the native runtime.
pub type RelayChainSpec = sc_service::GenericChainSpec<RelayChainExtensions>;

#[allow(clippy::unwrap_used)]
/// Generates the Frequency Paseo chain spec from the raw json
pub fn load_frequency_paseo_spec() -> ChainSpec {
	ChainSpec::from_json_bytes(
		&include_bytes!("../../../../resources/frequency-paseo.raw.json")[..],
	)
	.unwrap()
}

// TODO: Remove once on a Polkadot-SDK with Paseo
#[allow(clippy::unwrap_used)]
/// Generates the Paseo Relay chain spec from the json
pub fn load_paseo_spec() -> RelayChainSpec {
	RelayChainSpec::from_json_bytes(&include_bytes!("../../../../resources/paseo.json")[..])
		.unwrap()
}

// TODO: Remove once on a Polkadot-SDK with Paseo-Local
#[allow(clippy::unwrap_used)]
/// Generates the Paseo-Local Relay chain spec from the json
pub fn load_paseo_local_spec() -> RelayChainSpec {
	RelayChainSpec::from_json_bytes(&include_bytes!("../../../../resources/paseo-local.json")[..])
		.unwrap()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
fn template_session_keys(keys: AuraId) -> frequency_runtime::SessionKeys {
	frequency_runtime::SessionKeys { aura: keys }
}

/// Generates the chain spec for a local testnet
pub fn local_paseo_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let properties =
		get_properties(FREQUENCY_LOCAL_TOKEN, TOKEN_DECIMALS as u32, Ss58Prefix::get().into());

	ChainSpec::builder(
		frequency_runtime::wasm_binary_unwrap(),
		Extensions {
			relay_chain: "paseo-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
	.with_name("Frequency Local Testnet")
	.with_protocol_id("frequency-paseo-local")
	.with_properties(properties)
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name("paseo-testnet")
	.build()
}
