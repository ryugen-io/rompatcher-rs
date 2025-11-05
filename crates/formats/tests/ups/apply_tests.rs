//! Tests for UPS patch application

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ups::UpsPatcher;

/// Create minimal valid UPS patch for testing
fn create_test_patch() -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x8A); // Input size = 10
    patch.push(0x8A); // Output size = 10
    patch.push(0x80); // Relative offset 0
    patch.push(0xFF); // XOR with 0xFF
    patch.push(0x00); // Terminator

    let input_rom = vec![0u8; 10];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let mut output_rom = input_rom;
    output_rom[0] = 0xFF;
    let output_crc = crc32fast::hash(&output_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    patch
}

#[test]
fn test_apply_simple_xor() {
    let patcher = UpsPatcher;
    let patch = create_test_patch();
    let mut rom = vec![0u8; 10];

    assert!(patcher.apply(&mut rom, &patch).is_ok());
    assert_eq!(rom[0], 0xFF);
    assert_eq!(rom[1], 0x00);
}

#[test]
fn test_apply_multiple_xor_records() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x8A); // Input size = 10
    patch.push(0x8A); // Output size = 10

    // First XOR: at offset 0, change to 0xAA
    patch.push(0x80); // Relative offset 0
    patch.push(0xAA); // XOR data
    patch.push(0x00); // Terminator

    // Second XOR: at offset 4 (relative offset from pos 2 = 2)
    patch.push(0x82); // Relative offset 2
    patch.push(0xBB); // XOR data
    patch.push(0x00); // Terminator

    let input_rom = vec![0u8; 10];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let mut output_rom = input_rom.clone();
    output_rom[0] = 0xAA;
    output_rom[4] = 0xBB;
    let output_crc = crc32fast::hash(&output_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = UpsPatcher;
    let mut rom = vec![0u8; 10];
    assert!(patcher.apply(&mut rom, &patch).is_ok());
    assert_eq!(rom[0], 0xAA);
    assert_eq!(rom[4], 0xBB);
}

#[test]
fn test_apply_rom_resize() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x8A); // Input size = 10
    patch.push(0x94); // Output size = 20

    patch.push(0x8F); // Relative offset 15
    patch.push(0xCC);
    patch.push(0x00);

    let input_rom = vec![0u8; 10];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let mut output_rom = vec![0u8; 20];
    output_rom[15] = 0xCC;
    let output_crc = crc32fast::hash(&output_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = UpsPatcher;
    let mut rom = vec![0u8; 10];
    assert!(patcher.apply(&mut rom, &patch).is_ok());
    assert_eq!(rom.len(), 20);
    assert_eq!(rom[15], 0xCC);
}

#[test]
fn test_apply_multi_byte_xor() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x8A); // Input size = 10
    patch.push(0x8A); // Output size = 10

    patch.push(0x80); // Relative offset 0
    patch.push(0xFF);
    patch.push(0xEE);
    patch.push(0xDD);
    patch.push(0x00); // Terminator

    let input_rom = vec![0u8; 10];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let mut output_rom = input_rom.clone();
    output_rom[0] = 0xFF;
    output_rom[1] = 0xEE;
    output_rom[2] = 0xDD;
    let output_crc = crc32fast::hash(&output_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = UpsPatcher;
    let mut rom = vec![0u8; 10];
    assert!(patcher.apply(&mut rom, &patch).is_ok());
    assert_eq!(rom[0], 0xFF);
    assert_eq!(rom[1], 0xEE);
    assert_eq!(rom[2], 0xDD);
}

#[test]
fn test_verify_input_rom() {
    let patch = create_test_patch();
    let input_rom = vec![0u8; 10];
    assert!(UpsPatcher::verify(&input_rom, &patch, None).is_ok());
}

#[test]
fn test_verify_output_rom() {
    let patch = create_test_patch();
    let mut output_rom = vec![0u8; 10];
    output_rom[0] = 0xFF;
    assert!(UpsPatcher::verify(&[], &patch, Some(&output_rom)).is_ok());
}

#[test]
fn test_verify_wrong_input_checksum() {
    let patch = create_test_patch();
    let wrong_input = vec![0xFF; 10];
    assert!(UpsPatcher::verify(&wrong_input, &patch, None).is_err());
}

#[test]
fn test_verify_wrong_output_checksum() {
    let patch = create_test_patch();
    let wrong_output = vec![0xAA; 10];
    assert!(UpsPatcher::verify(&[], &patch, Some(&wrong_output)).is_err());
}
