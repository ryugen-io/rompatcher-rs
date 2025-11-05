//! UPS patcher implementation

use super::{constants::*, helpers, varint};
use rom_patcher_core::{PatchError, PatchFormat, PatchMetadata, PatchType, Result};

pub struct UpsPatcher;

impl PatchFormat for UpsPatcher {
    fn can_handle(data: &[u8]) -> bool {
        data.len() >= MAGIC_SIZE && &data[..MAGIC_SIZE] == MAGIC
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        // Validate magic
        if patch.len() < MAGIC_SIZE || &patch[..MAGIC_SIZE] != MAGIC {
            return Err(PatchError::InvalidFormat("Invalid UPS magic".to_string()));
        }

        // Parse header
        let (_input_size, output_size, mut offset) = helpers::parse_header(patch)?;

        // Resize ROM to output size
        rom.resize(output_size as usize, 0);

        // Process XOR records
        let mut rom_pos: usize = 0;

        while offset < patch.len() - FOOTER_SIZE {
            // Read relative offset
            let (relative_offset, bytes_read) = varint::decode(&patch[offset..])?;
            offset += bytes_read;
            rom_pos += relative_offset as usize;

            // Read XOR data until 0x00 terminator
            while offset < patch.len() && patch[offset] != 0x00 {
                if rom_pos >= rom.len() {
                    return Err(PatchError::InvalidFormat(
                        "XOR record exceeds ROM size".to_string(),
                    ));
                }

                // Apply XOR: output = input XOR patch_data
                rom[rom_pos] ^= patch[offset];
                rom_pos += 1;
                offset += 1;
            }

            // Skip terminator
            if offset >= patch.len() {
                return Err(PatchError::InvalidFormat(
                    "Missing XOR record terminator".to_string(),
                ));
            }
            offset += 1; // Skip 0x00
            rom_pos += 1; // Skip 1 byte in ROM
        }

        Ok(())
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        // Validate magic
        if patch.len() < MAGIC_SIZE || &patch[..MAGIC_SIZE] != MAGIC {
            return Err(PatchError::InvalidFormat("Invalid UPS magic".to_string()));
        }

        // Parse header
        let (input_size, output_size, _offset) = helpers::parse_header(patch)?;

        // Extract checksums
        let input_crc = u32::from_le_bytes([
            patch[patch.len() - 12],
            patch[patch.len() - 11],
            patch[patch.len() - 10],
            patch[patch.len() - 9],
        ]);
        let output_crc = u32::from_le_bytes([
            patch[patch.len() - 8],
            patch[patch.len() - 7],
            patch[patch.len() - 6],
            patch[patch.len() - 5],
        ]);

        Ok(PatchMetadata {
            patch_type: PatchType::Ups,
            source_size: Some(input_size as usize),
            target_size: Some(output_size as usize),
            source_checksum: Some(input_crc.to_le_bytes().to_vec()),
            target_checksum: Some(output_crc.to_le_bytes().to_vec()),
            extra: Vec::new(),
        })
    }

    fn validate(patch: &[u8]) -> Result<()> {
        // Check magic
        if patch.len() < MAGIC_SIZE || &patch[..MAGIC_SIZE] != MAGIC {
            return Err(PatchError::InvalidFormat("Invalid UPS magic".to_string()));
        }

        // Check minimum size
        if patch.len() < MAGIC_SIZE + FOOTER_SIZE {
            return Err(PatchError::InvalidFormat("Patch too small".to_string()));
        }

        // Validate patch CRC32
        helpers::validate_patch_crc(patch)?;

        Ok(())
    }

    fn verify(rom: &[u8], patch: &[u8], target: Option<&[u8]>) -> Result<()> {
        if let Some(target_rom) = target {
            // Verify output ROM
            helpers::validate_output_crc(target_rom, patch)?;
        } else {
            // Verify input ROM
            helpers::validate_input_crc(rom, patch)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_rejects_invalid_magic() {
        let invalid_patch = b"NOPE";
        assert!(UpsPatcher::validate(invalid_patch).is_err());
    }

    #[test]
    fn test_validate_rejects_too_small() {
        let invalid_patch = b"UPS1";
        assert!(UpsPatcher::validate(invalid_patch).is_err());
    }
}
