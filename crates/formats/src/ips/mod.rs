//! IPS (International Patching System) format implementation
//!
//! IPS is one of the oldest and most widely used patch formats.
//!
//! ## Format Specification
//!
//! - **Header**: "PATCH" (5 bytes)
//! - **Records**: `[offset (3 bytes BE)][size (2 bytes BE)][data (size bytes)]`
//!   - RLE encoding: size = 0, followed by `[rle_size (2 bytes BE)][value (1 byte)]`
//! - **Footer**: "EOF" (3 bytes)
//!
//! ## Limitations
//!
//! - Maximum ROM size: 16 MB (24-bit addressing)
//! - No checksums or validation
//! - No metadata storage

use rom_patcher_core::{PatchFormat, PatchMetadata, Result};

mod apply;
mod constants;
mod io;
mod metadata;
mod validate;

pub use constants::{MAX_RECORD_SIZE, MAX_ROM_SIZE};

/// IPS format patcher
pub struct IpsPatcher;

impl PatchFormat for IpsPatcher {
    fn can_handle(data: &[u8]) -> bool {
        apply::can_handle(data)
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        apply::apply(rom, patch)
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        metadata::extract(patch)
    }

    fn validate(patch: &[u8]) -> Result<()> {
        validate::validate(patch)
    }
}
