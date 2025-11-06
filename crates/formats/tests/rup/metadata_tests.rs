//! Tests for RUP metadata extraction

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::rup::RupPatcher;
use std::fs;

#[test]
fn test_metadata_from_real_patch() {
    let patch = fs::read("../../test_files/rup/test.rup").expect("Failed to read RUP patch");
    let metadata = RupPatcher::metadata(&patch).expect("Failed to extract metadata");

    assert_eq!(metadata.patch_type, rom_patcher_core::PatchType::Rup);
    assert!(metadata.source_size.is_some());
    assert!(metadata.target_size.is_some());
}

#[test]
fn test_metadata_has_extra_fields() {
    let patch = fs::read("../../test_files/rup/test.rup").expect("Failed to read RUP patch");
    let metadata = RupPatcher::metadata(&patch).expect("Failed to extract metadata");

    // RUP patches can have author, title, version, etc.
    assert!(!metadata.extra.is_empty(), "RUP should have extra metadata");
}

#[test]
fn test_metadata_invalid_patch() {
    let invalid_patch = b"NOTRUP";
    let result = RupPatcher::metadata(invalid_patch);

    assert!(result.is_err());
}

#[test]
fn test_metadata_truncated_patch() {
    let patch = b"NINJA2";
    let result = RupPatcher::metadata(patch);

    assert!(result.is_err());
}
