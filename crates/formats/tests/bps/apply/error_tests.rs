//! BPS error handling tests

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::bps::BpsPatcher;

#[test]
fn test_apply_invalid_patch() {
    let mut rom = vec![0x00; 10];
    let patch = b"NOTBPS";

    let patcher = BpsPatcher;
    assert!(patcher.apply(&mut rom, patch).is_err());
}

#[test]
fn test_apply_truncated_patch() {
    let mut rom = vec![0x00; 10];
    let patch = b"BPS1\x85\x83"; // Incomplete header

    let patcher = BpsPatcher;
    assert!(patcher.apply(&mut rom, patch).is_err());
}
