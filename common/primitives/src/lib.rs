//! # MRC Primitives
//!
//! Primitives package contains many of the structs and trait implementations
//! for Pallets and utilities that need to be shared across packages

#![cfg_attr(not(feature = "std"), no_std)]
// Strong Documentation Lints
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_codeblock_attributes)]

/// Structs and traits for the Messages pallet.
pub mod messages;
/// Structs and traits for the MSA pallet.
pub mod msa;
/// Structs and traits specifically for RPC calls.
#[cfg(feature = "std")]
/// export rpc primitive types.
pub mod rpc;
/// Structs and traits for the Schema pallet
pub mod schema;
/// Structs and traits for the utility package.
pub mod utils;
