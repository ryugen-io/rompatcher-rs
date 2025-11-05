//! Tests for BPS metadata extraction

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::bps::BpsPatcher;

fn write_varint(buf: &mut Vec<u8>, mut data: u64) {
    loop {
        let x = (data & 0x7f) as u8;
        data >>= 7;
        if data == 0 {
            buf.push(0x80 | x);
            break;
        }
        buf.push(x);
        data -= 1;
    }
}

fn create_bps_patch_with_metadata(
    source_size: usize,
    target_size: usize,
    metadata: &str,
) -> Vec<u8> {
    let mut patch = Vec::new();

    // Magic header
    patch.extend_from_slice(b"BPS1");

    // Source size (varint)
    write_varint(&mut patch, source_size as u64);

    // Target size (varint)
    write_varint(&mut patch, target_size as u64);

    // Metadata
    write_varint(&mut patch, metadata.len() as u64);
    patch.extend_from_slice(metadata.as_bytes());

    // No actions

    // Checksums
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32

    // Patch CRC32
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    patch
}

#[test]
fn test_metadata_simple() {
    let patch = create_bps_patch_with_metadata(1024, 2048, "");

    let metadata = BpsPatcher::metadata(&patch).unwrap();

    assert_eq!(metadata.source_size, Some(1024));
    assert_eq!(metadata.target_size, Some(2048));
}

#[test]
fn test_metadata_with_info() {
    let patch = create_bps_patch_with_metadata(512, 1024, "Test patch v1.0");

    let metadata = BpsPatcher::metadata(&patch).unwrap();

    assert_eq!(metadata.source_size, Some(512));
    assert_eq!(metadata.target_size, Some(1024));
}

#[test]
fn test_metadata_invalid_patch() {
    let invalid_patch = b"NOTBPS";
    let result = BpsPatcher::metadata(invalid_patch);

    assert!(result.is_err());
}
