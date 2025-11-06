//! IPS error handling tests

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ebp::EbpPatcher;

#[test]
fn test_apply_invalid_patch() {
    let mut rom = vec![0x00; 10];
    let patch = b"NOTVALID";

    let patcher = EbpPatcher;
    assert!(patcher.apply(&mut rom, patch).is_err());
}

#[test]
fn test_apply_truncated_patch() {
    let mut rom = vec![0x00; 10];
    // Incomplete record
    let patch = b"PATCH\x00\x00\x05\x00";

    let patcher = EbpPatcher;
    assert!(patcher.apply(&mut rom, patch).is_err());
}
