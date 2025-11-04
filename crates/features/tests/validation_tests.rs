//! Validation feature tests

use rom_patcher_features::validation::{HashAlgorithm, ValidationFeature, Validator};

#[test]
fn test_crc32_empty() {
    let data = b"";
    let validator = Validator::new();
    let hash = validator.compute_hash(data, HashAlgorithm::Crc32);
    assert_eq!(hash, vec![0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn test_crc32_known_values() {
    let validator = Validator::new();

    // Test "123456789"
    let data = b"123456789";
    let hash = validator.compute_hash(data, HashAlgorithm::Crc32);
    assert_eq!(hash, vec![0xCB, 0xF4, 0x39, 0x26]);

    // Test "The quick brown fox jumps over the lazy dog"
    let data = b"The quick brown fox jumps over the lazy dog";
    let hash = validator.compute_hash(data, HashAlgorithm::Crc32);
    assert_eq!(hash, vec![0x41, 0x4F, 0xA3, 0x39]);
}

#[test]
fn test_validation() {
    let data = b"test data";
    let validator = Validator::new();

    let hash = validator.compute_hash(data, HashAlgorithm::Crc32);
    assert!(
        validator
            .validate_checksum(data, &hash, HashAlgorithm::Crc32)
            .is_ok()
    );

    let wrong_hash = vec![0x00, 0x00, 0x00, 0x00];
    assert!(
        validator
            .validate_checksum(data, &wrong_hash, HashAlgorithm::Crc32)
            .is_err()
    );
}
