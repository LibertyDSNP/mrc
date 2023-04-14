//! # Suffix Generator
//!
//! `suffix_generator` provides a `SuffixGenerator` struct to generate unique suffix sequences for a given range
//! and seed, excluding already used suffixes.

use core::hash::Hasher;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use sp_std::vec::Vec;
use twox_hash::XxHash64;

/// Generate a unique, shuffled suffix iterator.
///
/// # Returns
///
/// An iterator over the unique, shuffled sequence of suffixes.
///
/// # Examples
///
/// ```
///  use pallet_handles::suffix::generate_unique_suffixes;
///
/// let min = 100;
/// let max = 150;
/// let canonical_base = "myhandle";
///
/// let lazy_sequence = generate_unique_suffixes(min, max, canonical_base);
/// let suffixes: Vec<u16> = lazy_sequence.collect();
/// ```
///
/// This will output a unique, shuffled sequence of suffixes.
/// Note: This is a lazy iterator, so it will not be evaluated until it is consumed.
pub fn generate_unique_suffixes(
	min: u16,
	max: u16,
	canonical_base: &str,
) -> impl Iterator<Item = u16> + '_ {
	let seed = generate_seed(canonical_base);
	let mut rng = SmallRng::seed_from_u64(seed);

	let mut indices: Vec<u16> = (min..=max).collect();
	let min = min as usize;
	let max = max as usize;
	(min..=max).rev().map(move |i| {
		let j = rng.gen_range(min..=i);
		indices.swap(i - min, j - min);
		indices[i - min]
	})
}

/// Generate a seed from a unique canonical base handle.
///
/// # Arguments
///
/// * `canonical_base` - The canonical base as a string slice.
///
/// # Returns
///
/// A 64-bit seed.
///
/// # Examples
/// ```
///  use pallet_handles::suffix::generate_seed;
///
/// let canonical_base = "myuser";
///
/// let seed = generate_seed(canonical_base);
/// ```
pub fn generate_seed(canonical_base: &str) -> u64 {
	let mut hasher = XxHash64::with_seed(0);
	hasher.write(canonical_base.as_bytes());
	let value_bytes: [u8; 4] = [0; 4];
	hasher.write(&value_bytes);
	hasher.finish()
}
