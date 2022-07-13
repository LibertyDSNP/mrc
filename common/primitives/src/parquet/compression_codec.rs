use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_std::prelude::*;

/// Compression codecs: https://github.com/apache/parquet-format/blob/master/LogicalTypes.md
#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, MaxEncodedLen, Serialize, Deserialize)]
pub enum CompressionCodec {
	/// The compression used in the Bloom filter
	/// NOTE: more research required here
  Uncompressed
}

impl Default for CompressionCodec {
	fn default() -> Self {
		Self::Uncompressed
	}
}
