//! Core traits and types for ROM patching
//!
//! This crate provides the foundational abstractions for implementing
//! various ROM patch formats and extensible features.

pub mod error;
pub mod format;
pub mod types;

pub use error::{PatchError, Result};
pub use format::PatchFormat;
pub use types::{PatchMetadata, PatchType};
