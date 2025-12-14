//! PPF validation tests.

use rom_patcher_core::{PatchError, PatchFormat};
use rom_patcher_formats::ppf::PpfPatcher;

#[test]

fn test_validate_valid_ppf3() {
    let mut patch_data = Vec::new();

    patch_data.extend_from_slice(b"PPF30");

    patch_data.push(0x02); // Encoding

    patch_data.extend_from_slice(&[0u8; 50]); // Description

    patch_data.push(0x00); // Image Type

    patch_data.push(0x00); // Block Check (False)

    patch_data.push(0x00); // Undo Data (False)

    patch_data.push(0x00); // Dummy

    // Record

    patch_data.extend_from_slice(&0u64.to_le_bytes());

    patch_data.push(0x01);

    patch_data.push(0xAA);

    assert!(PpfPatcher::validate(&patch_data).is_ok());
}

#[test]

fn test_validate_truncated_ppf3() {
    let mut patch_data = Vec::new();

    patch_data.extend_from_slice(b"PPF30");

    patch_data.push(0x02); // Encoding

    // Truncate in description

    patch_data.extend_from_slice(&[0u8; 10]);

    assert!(matches!(
        PpfPatcher::validate(&patch_data),
        Err(PatchError::CorruptedData)
    ));
}
