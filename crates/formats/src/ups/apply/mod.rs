//! UPS patch application

use super::constants::*;
use super::helpers::*;
use super::varint;
use rom_patcher_core::{PatchError, Result};

/// Apply a UPS patch to a ROM
pub fn apply(rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
    // Validate magic
    if patch.len() < MAGIC_SIZE || &patch[..MAGIC_SIZE] != MAGIC {
        return Err(PatchError::InvalidFormat("Invalid UPS magic".to_string()));
    }

    // Parse header
    let (_input_size, output_size, mut offset) = parse_header(patch)?;

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
