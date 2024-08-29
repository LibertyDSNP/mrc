use crate::development_genesis::*;
/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &sp_genesis_builder::PresetId) -> Option<sp_std::vec::Vec<u8>> {
	let patch = match id.try_into() {
		Ok("development") => development_genesis(
			development_invulnerables(),
			development_root(),
			development_endowed_accounts(),
			development_council_members(),
			development_technical_committee_members(),
			1000.into(),
		),
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}
