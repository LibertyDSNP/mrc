use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_std::prelude::*;

/// Parquet temporal types: https://github.com/apache/parquet-format/blob/master/LogicalTypes.md
#[derive(
	Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, MaxEncodedLen, Serialize, Deserialize,
)]
pub enum ParquetTemporalType {
	/// Parquet dates
	Date,
	/// Parquet intervals
	Interval,
	/// Time
	Time(ParquetTime),
	/// Timestamps
	Timestamp(ParquetTimestamp),
}

impl Default for ParquetTemporalType {
	fn default() -> Self {
		Self::Timestamp(ParquetTimestamp::default())
	}
}

/// Parquet time: https://github.com/apache/parquet-format/blob/master/LogicalTypes.md
#[derive(
	Default,
	Clone,
	Encode,
	Decode,
	PartialEq,
	Debug,
	TypeInfo,
	Eq,
	MaxEncodedLen,
	Serialize,
	Deserialize,
)]
pub struct ParquetTime {
	is_adjusted_to_utc: bool,
	unit: ParquetTimeUnit,
}

/// Parquet timestamps: https://github.com/apache/parquet-format/blob/master/LogicalTypes.md
#[derive(
	Default,
	Clone,
	Encode,
	Decode,
	PartialEq,
	Debug,
	TypeInfo,
	Eq,
	MaxEncodedLen,
	Serialize,
	Deserialize,
)]
pub struct ParquetTimestamp {
	is_adjusted_to_utc: bool,
	unit: ParquetTimeUnit,
}

/// Units of time
#[derive(
	Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, MaxEncodedLen, Serialize, Deserialize,
)]
enum ParquetTimeUnit {
	/// milliseconds
	MILLIS,
	/// microseconds
	MICROS,
	/// nanoseconds
	NANOS,
}

impl Default for ParquetTimeUnit {
	fn default() -> Self {
		Self::MILLIS
	}
}
