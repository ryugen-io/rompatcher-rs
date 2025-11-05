//! APS N64 patch application

use super::constants::*;
use super::helpers::{parse_header, validate_source_rom};
use rom_patcher_core::{PatchError, Result};

/// Apply APS N64 patch to ROM
pub fn apply(rom: &[u8], patch: &[u8]) -> Result<Vec<u8>> {
    let (header, mut offset) = parse_header(patch)?;

    if header.header_type != HEADER_TYPE_N64 {
        return Err(PatchError::InvalidFormat(
            "Not an APS N64 patch".to_string(),
        ));
    }

    let mut output = vec![0u8; header.output_size as usize];

    let copy_len = rom.len().min(output.len());
    output[..copy_len].copy_from_slice(&rom[..copy_len]);

    while offset < patch.len() {
        if offset + 5 > patch.len() {
            return Err(PatchError::UnexpectedEof(
                "Incomplete record header".to_string(),
            ));
        }

        let record_offset = u32::from_le_bytes([
            patch[offset],
            patch[offset + 1],
            patch[offset + 2],
            patch[offset + 3],
        ]) as usize;
        offset += 4;

        let length = patch[offset];
        offset += 1;

        if length == RECORD_RLE {
            if offset + 2 > patch.len() {
                return Err(PatchError::UnexpectedEof(
                    "Incomplete RLE record".to_string(),
                ));
            }

            let byte_value = patch[offset];
            offset += 1;
            let count = patch[offset] as usize;
            offset += 1;

            if record_offset + count > output.len() {
                return Err(PatchError::OutOfBounds {
                    offset: record_offset,
                    rom_size: output.len(),
                });
            }

            for i in 0..count {
                output[record_offset + i] = byte_value;
            }
        } else {
            let data_len = length as usize;
            if offset + data_len > patch.len() {
                return Err(PatchError::UnexpectedEof(
                    "Incomplete simple record".to_string(),
                ));
            }

            if record_offset + data_len > output.len() {
                return Err(PatchError::OutOfBounds {
                    offset: record_offset,
                    rom_size: output.len(),
                });
            }

            output[record_offset..record_offset + data_len]
                .copy_from_slice(&patch[offset..offset + data_len]);
            offset += data_len;
        }
    }

    Ok(output)
}

/// Verify source ROM matches N64 header requirements
pub fn verify(rom: &[u8], patch: &[u8]) -> Result<()> {
    let (header, _) = parse_header(patch)?;

    if let Some(n64_header) = header.n64_header
        && !validate_source_rom(rom, &n64_header)
    {
        return Err(PatchError::ChecksumMismatch {
            expected: 0,
            actual: 0,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_patch_with_records() -> Vec<u8> {
        let mut patch = Vec::new();
        patch.extend_from_slice(MAGIC);
        patch.push(HEADER_TYPE_N64);
        patch.push(0x00);
        patch.extend_from_slice(&[0u8; DESCRIPTION_LEN]);

        // N64 header
        patch.push(0x01);
        patch.extend_from_slice(b"NTE");
        patch.extend_from_slice(&[0u8; N64_CRC_LEN]);
        patch.extend_from_slice(&[0u8; N64_PAD_LEN]);

        // Output size: 1024 bytes
        patch.extend_from_slice(&1024u32.to_le_bytes());

        // Add a simple record at offset 0x100
        patch.extend_from_slice(&0x100u32.to_le_bytes()); // Offset
        patch.push(4); // Length
        patch.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]); // Data

        // Add an RLE record at offset 0x200
        patch.extend_from_slice(&0x200u32.to_le_bytes()); // Offset
        patch.push(RECORD_RLE); // RLE marker
        patch.push(0xFF); // Byte value
        patch.push(10); // Count

        patch
    }

    #[test]
    fn test_apply_with_records() {
        let rom = vec![0u8; 512];
        let patch = create_test_patch_with_records();

        let result = apply(&rom, &patch);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert_eq!(output.len(), 1024);

        // Check simple record
        assert_eq!(&output[0x100..0x104], &[0xDE, 0xAD, 0xBE, 0xEF]);

        // Check RLE record
        assert_eq!(&output[0x200..0x20A], &[0xFF; 10]);
    }
}
