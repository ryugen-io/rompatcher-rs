//! Test helpers for IPS checksum validation

use std::path::PathBuf;

pub fn test_rom_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_files/ips")
        .join(name)
}
