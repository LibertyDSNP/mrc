extern crate alloc;
use crate::offchain_storage::data::MSAPublicKeyData;
use codec::Encode;
use common_primitives::offchain as offchain_common;
use sp_runtime::offchain::{storage::StorageRetrievalError, StorageKind};

/// Process MSA key event and update offchain storage
pub fn process_msa_key_event(
	msa_id: u64,
	key: &[u8],
	value: &[u8],
) -> Result<(), StorageRetrievalError> {
	let mut msa_keys = offchain_common::get_index_value::<MSAPublicKeyData>(
		StorageKind::PERSISTENT,
		msa_id.to_string().as_bytes(),
	);
	let msa_key_data = msa_keys.unwrap_or_default();
	let mut msa_key_map = msa_key_data.0;
	msa_key_map.insert(msa_id, key.to_vec());
	let msa_keys_updated = MSAPublicKeyData(msa_key_map).encode();
	offchain_common::set_index_value(msa_id.to_string().as_bytes(), msa_keys_updated.as_slice());
	Ok(())
}
