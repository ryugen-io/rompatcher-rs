//! APS N64 helper functions

use super::constants::*;
use rom_patcher_core::{PatchError, Result};

/// N64-specific header data
#[derive(Debug, Clone)]
pub struct N64Header {
    pub original_format: u8,
    pub cart_id: [u8; N64_CART_ID_LEN],
    pub crc: [u8; N64_CRC_LEN],
    pub pad: [u8; N64_PAD_LEN],
}

/// APS patch header
#[derive(Debug, Clone)]
pub struct ApsHeader {
    pub header_type: u8,
    pub encoding_method: u8,
    pub description: String,
    pub n64_header: Option<N64Header>,
    pub output_size: u32,
}

/// Parse APS header from patch data
pub fn parse_header(patch: &[u8]) -> Result<(ApsHeader, usize)> {
    if patch.len() < MIN_PATCH_SIZE {
        return Err(PatchError::InvalidFormat(
            "Patch file too small".to_string(),
        ));
    }

    let mut offset = MAGIC_LEN;

    // Read header type and encoding method
    let header_type = patch[offset];
    offset += 1;
    let encoding_method = patch[offset];
    offset += 1;

    // Read description (50 bytes, null-terminated)
    let desc_bytes = &patch[offset..offset + DESCRIPTION_LEN];
    let description = String::from_utf8_lossy(desc_bytes)
        .trim_end_matches('\0')
        .to_string();
    offset += DESCRIPTION_LEN;

    // Parse N64-specific header if type is N64
    let n64_header = if header_type == HEADER_TYPE_N64 {
        if patch.len() < offset + N64_HEADER_SIZE + 4 {
            return Err(PatchError::InvalidFormat(
                "Patch file too small for N64 header".to_string(),
            ));
        }

        let original_format = patch[offset];
        offset += 1;

        let mut cart_id = [0u8; N64_CART_ID_LEN];
        cart_id.copy_from_slice(&patch[offset..offset + N64_CART_ID_LEN]);
        offset += N64_CART_ID_LEN;

        let mut crc = [0u8; N64_CRC_LEN];
        crc.copy_from_slice(&patch[offset..offset + N64_CRC_LEN]);
        offset += N64_CRC_LEN;

        let mut pad = [0u8; N64_PAD_LEN];
        pad.copy_from_slice(&patch[offset..offset + N64_PAD_LEN]);
        offset += N64_PAD_LEN;

        Some(N64Header {
            original_format,
            cart_id,
            crc,
            pad,
        })
    } else {
        None
    };

    // Read output size (little-endian u32)
    let output_size = u32::from_le_bytes([
        patch[offset],
        patch[offset + 1],
        patch[offset + 2],
        patch[offset + 3],
    ]);
    offset += 4;

    Ok((
        ApsHeader {
            header_type,
            encoding_method,
            description,
            n64_header,
            output_size,
        },
        offset,
    ))
}

/// Validate source ROM against N64 header
pub fn validate_source_rom(rom: &[u8], n64_header: &N64Header) -> bool {
    // Check minimum ROM size
    if rom.len() < N64_CRC_OFFSET + N64_CRC_LEN {
        return false;
    }

    // Validate cart ID
    if rom[N64_CART_ID_OFFSET..N64_CART_ID_OFFSET + N64_CART_ID_LEN] != n64_header.cart_id {
        return false;
    }

    // Validate CRC
    if rom[N64_CRC_OFFSET..N64_CRC_OFFSET + N64_CRC_LEN] != n64_header.crc {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_header() {
        let mut patch = vec![0u8; MIN_PATCH_SIZE];
        patch[..MAGIC_LEN].copy_from_slice(MAGIC);
        patch[MAGIC_LEN] = 0x00; // header_type (not N64)
        patch[MAGIC_LEN + 1] = 0x00; // encoding_method

        // Output size at end (LE)
        let offset = MAGIC_LEN + 2 + DESCRIPTION_LEN;
        patch[offset..offset + 4].copy_from_slice(&1024u32.to_le_bytes());

        let result = parse_header(&patch);
        assert!(result.is_ok());

        let (header, _) = result.unwrap();
        assert_eq!(header.output_size, 1024);
        assert!(header.n64_header.is_none());
    }
}
