//! PPF (PlayStation Patch Format) validation.
//!
//! This module provides functionality to validate PPF patch files.

use crate::ppf::constants::*;
use rom_patcher_core::{PatchError, Result};

/// Checks if the provided data can be handled as a PPF patch.
///
/// # Arguments
///
/// * `data` - The byte slice to check.
///
/// # Returns
///
/// * `bool` - True if the data appears to be a PPF patch, false otherwise.
pub fn can_handle(data: &[u8]) -> bool {
    if data.len() < 5 {
        return false;
    }

    &data[0..5] == PPF1_HEADER || &data[0..5] == PPF2_HEADER || &data[0..5] == PPF3_HEADER
}

/// Validates a PPF patch file.
///
/// # Arguments
///
/// * `patch` - The patch data.
///
/// # Returns
///
/// * `Result<()>` - Ok if the patch is valid, an error otherwise.
pub fn validate_patch(patch: &[u8]) -> Result<()> {
    if !can_handle(patch) {
        return Err(PatchError::InvalidFormat("Not a PPF patch".to_string()));
    }

    // Additional validation logic for PPF can be added here,
    // e.g., checking patch file size, integrity, etc.
    Ok(())
}
