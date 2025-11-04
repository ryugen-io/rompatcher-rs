//! Validation features for ROM and patch integrity checking
//!
//! Provides checksum and hash verification capabilities for ensuring
//! data integrity during patching operations.

pub mod algorithms;
mod trait_def;
mod types;
mod validator;

pub use trait_def::ValidationFeature;
pub use types::HashAlgorithm;
pub use validator::Validator;
