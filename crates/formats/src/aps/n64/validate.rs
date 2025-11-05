//! APS N64 validation functions

use super::constants::*;
use super::helpers::parse_header;
use rom_patcher_core::{PatchError, Result};

/// Check if data is a valid APS N64 patch
pub fn can_handle(data: &[u8]) -> bool {
    if data.len() < MAGIC_LEN {
        return false;
    }

    if &data[..MAGIC_LEN] != MAGIC {
        return false;
    }

    parse_header(data).is_ok()
}

/// Validate patch structure
pub fn validate(patch: &[u8]) -> Result<()> {
    if patch.len() < MIN_PATCH_SIZE {
        return Err(PatchError::InvalidFormat(
            "Patch file too small".to_string(),
        ));
    }

    if &patch[..MAGIC_LEN] != MAGIC {
        return Err(PatchError::InvalidMagic {
            expected: MAGIC.to_vec(),
            actual: patch.get(..MAGIC_LEN).unwrap_or(&[]).to_vec(),
        });
    }

    let (header, mut offset) = parse_header(patch)?;

    if header.header_type != HEADER_TYPE_N64 {
        return Err(PatchError::InvalidFormat(
            "Not an APS N64 patch".to_string(),
        ));
    }

    while offset < patch.len() {
        if offset + 5 > patch.len() {
            return Err(PatchError::UnexpectedEof(
                "Incomplete record header".to_string(),
            ));
        }

        offset += 4;
        let length = patch[offset];
        offset += 1;

        if length == RECORD_RLE {
            if offset + 2 > patch.len() {
                return Err(PatchError::UnexpectedEof(
                    "Incomplete RLE record".to_string(),
                ));
            }
            offset += 2;
        } else {
            if offset + length as usize > patch.len() {
                return Err(PatchError::UnexpectedEof(
                    "Incomplete simple record".to_string(),
                ));
            }
            offset += length as usize;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_minimal_patch() -> Vec<u8> {
        let mut patch = Vec::new();
        patch.extend_from_slice(MAGIC); // Magic
        patch.push(HEADER_TYPE_N64); // Header type
        patch.push(0x00); // Encoding method
        patch.extend_from_slice(&[0u8; DESCRIPTION_LEN]); // Description

        // N64 header
        patch.push(0x01); // Original format
        patch.extend_from_slice(b"NTE"); // Cart ID
        patch.extend_from_slice(&[0u8; N64_CRC_LEN]); // CRC
        patch.extend_from_slice(&[0u8; N64_PAD_LEN]); // Padding

        // Output size (1024 bytes, LE)
        patch.extend_from_slice(&1024u32.to_le_bytes());

        patch
    }

    #[test]
    fn test_can_handle_valid() {
        let patch = create_minimal_patch();
        assert!(can_handle(&patch));
    }

    #[test]
    fn test_can_handle_invalid_magic() {
        let patch = b"INVALID";
        assert!(!can_handle(patch));
    }

    #[test]
    fn test_validate_minimal() {
        let patch = create_minimal_patch();
        assert!(validate(&patch).is_ok());
    }

    #[test]
    fn test_validate_too_small() {
        let patch = &[0u8; 10];
        assert!(validate(patch).is_err());
    }
}
