//! PPF (PlayStation Patch Format) metadata extraction.
//!
//! This module provides functionality to extract metadata from PPF patch files.

use crate::ppf::constants::*;
use rom_patcher_core::{PatchError, PatchMetadata, PatchType, Result};

/// Extracts metadata from a PPF patch.
///
/// # Arguments
///
/// * `patch` - The patch data.
///
/// # Returns
///
/// * `Result<PatchMetadata>` - The extracted metadata, or an error if
///   extraction fails.
pub fn extract_metadata(patch: &[u8]) -> Result<PatchMetadata> {
    if !super::validate::can_handle(patch) {
        return Err(PatchError::InvalidFormat("Not a PPF patch".to_string()));
    }

    let mut metadata = PatchMetadata::new(PatchType::Ppf);

    // Detect version
    if patch.len() >= 5 {
        if &patch[0..5] == PPF1_HEADER {
            metadata = metadata.with_extra("version".to_string(), "PPF1".to_string());
        } else if &patch[0..5] == PPF2_HEADER {
            metadata = metadata.with_extra("version".to_string(), "PPF2".to_string());
        } else if &patch[0..5] == PPF3_HEADER {
            metadata = metadata.with_extra("version".to_string(), "PPF3".to_string());
        }
    }

    Ok(metadata)
}
