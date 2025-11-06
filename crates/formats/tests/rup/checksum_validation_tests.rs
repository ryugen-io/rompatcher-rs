//! RUP MD5 checksum validation tests

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::rup::RupPatcher;
use std::fs;

#[test]
fn test_patch_file_integrity() {
    let patch = fs::read("../../test_files/rup/test.rup").expect("Failed to read RUP patch");

    // RUP validate() checks header integrity
    assert!(
        RupPatcher::validate(&patch).is_ok(),
        "RUP patch validation should succeed"
    );
}

#[test]
fn test_tekkaman_blade_patch() {
    let patch = fs::read("../../test_files/rup/test.rup").expect("Failed to read RUP patch");

    // Validate patch integrity
    assert!(
        RupPatcher::validate(&patch).is_ok(),
        "Tekkaman Blade RUP patch should pass validation"
    );

    // Extract metadata
    let metadata = RupPatcher::metadata(&patch).expect("Failed to extract metadata");

    assert!(
        metadata.source_size.is_some(),
        "Metadata should include source size"
    );
    assert!(
        metadata.target_size.is_some(),
        "Metadata should include target size"
    );

    // Check extracted metadata fields
    let author = metadata.extra.iter().find(|(k, _)| k == "author");
    assert!(author.is_some(), "Should have author metadata");

    let title = metadata.extra.iter().find(|(k, _)| k == "title");
    assert!(title.is_some(), "Should have title metadata");
}
