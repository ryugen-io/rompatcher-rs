//! RUP bidirectional/undo tests

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::rup::RupPatcher;
use std::fs;

#[test]
fn test_undo_capability() {
    // RUP is bidirectional - same patch can apply forward or undo
    let patch = fs::read("../../test_files/rup/test.rup").expect("Failed to read RUP patch");
    let mut rom = fs::read("../../test_files/rup/rom.sfc").expect("Failed to read ROM");
    let original_rom = rom.clone();

    let patcher = RupPatcher;

    // Apply forward
    patcher
        .apply(&mut rom, &patch)
        .expect("Forward apply should succeed");
    let patched_rom = rom.clone();

    // Apply again (should undo)
    patcher
        .apply(&mut rom, &patch)
        .expect("Undo apply should succeed");

    assert_eq!(
        rom, original_rom,
        "Second apply should restore original ROM"
    );

    // Apply third time (forward again)
    patcher
        .apply(&mut rom, &patch)
        .expect("Third apply should succeed");

    assert_eq!(rom, patched_rom, "Third apply should match first result");
}

#[test]
fn test_verify_with_target() {
    let patch = fs::read("../../test_files/rup/test.rup").expect("Failed to read RUP patch");
    let mut source = fs::read("../../test_files/rup/rom.sfc").expect("Failed to read ROM");

    let patcher = RupPatcher;
    patcher
        .apply(&mut source, &patch)
        .expect("Apply should succeed");

    let target = source.clone();
    let original = fs::read("../../test_files/rup/rom.sfc").expect("Failed to read ROM");

    // Verify target from source
    let result = RupPatcher::verify(&original, &patch, Some(&target));
    assert!(result.is_ok(), "Target verification should succeed");
}
