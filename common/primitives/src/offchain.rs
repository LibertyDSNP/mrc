use codec::{Decode, Encode};
use sp_runtime::offchain::storage::{StorageRetrievalError, StorageValueRef};
use sp_std::fmt::Debug;

/// Storage keys for offchain worker
/// THe tx_id is used to identify the transaction
const TX_ID: &[u8] = b"tx_id";
/// The lock is used to prevent the execution of the function
const DB_LOCK: &[u8] = b"lock";
/// The tx_id is used to identify the transaction in persistent storage
const DB_TX_ID: &[u8] = b"tx-id/";

/// Locks the execution of the function
#[derive(Debug)]
pub enum LockStatus {
	/// Lock is acquired
	Locked,
	/// Lock is released
	Release,
}

/// Locks the execution of the function
pub fn lock<F>(prefix: &[u8], f: F) -> LockStatus
where
	F: Fn(),
{
	// The key used to store the lock in the persistent storage
	let lock_key = [prefix, DB_LOCK].concat();
	let mut lock_storage = StorageValueRef::persistent(&lock_key);

	// Check the execution id in the persistent storage
	let exec_id_opt = StorageValueRef::persistent(DB_TX_ID).get();
	if let Ok(Some(exec_id)) = exec_id_opt {
		// The key used to store the execution id in the persistent storage
		let id_key = [prefix, TX_ID].concat();
		let id_storage = StorageValueRef::persistent(&id_key);

		// Check if the lock needs to be cleared due to a new execution id
		let need_to_clear_lock =
			id_storage.mutate(|id: Result<Option<[u8; 32]>, StorageRetrievalError>| {
				match id {
					Ok(Some(val)) => {
						if val != exec_id {
							// new id we need to clear lock because of first launch
							Ok(exec_id)
						} else {
							Err(())
						}
					},
					_ => {
						// no id we need to clear lock because of first launch
						Ok(exec_id)
					},
				}
			});

		if need_to_clear_lock.is_ok() {
			lock_storage.clear();
		}
	}

	// Try to acquire the lock
	let can_process = lock_storage.mutate(
		|is_locked: Result<Option<bool>, StorageRetrievalError>| match is_locked {
			Ok(Some(true)) => Err(()),
			_ => Ok(true),
		},
	);

	// If the lock is acquired, execute the function else return the lock status
	match can_process {
		Ok(true) => {
			f();
			lock_storage.clear();
			LockStatus::Release
		},
		_ => LockStatus::Locked,
	}
}

/// Wrapper for offchain get operations
pub fn get_index_value<V: Decode + Debug>(key: &[u8]) -> Result<V, StorageRetrievalError> {
	let indexed_value = get_impl::<V>(key);
	indexed_value
}

/// Sets a value by the key to offchain index
pub fn set_index_value<V>(key: &[u8], value: V)
where
	V: Encode + Debug,
{
	let oci_mem = StorageValueRef::persistent(key);
	oci_mem.set(&value);
}

/// Gets a value by the key from persistent storage
fn get_impl<V: Decode + Debug>(key: &[u8]) -> Result<V, StorageRetrievalError> {
	let oci_mem = StorageValueRef::persistent(key);
	if let Ok(Some(data)) = oci_mem.get::<V>() {
		return Ok(data)
	} else {
		return Err(StorageRetrievalError::Undecodable)
	}
}
