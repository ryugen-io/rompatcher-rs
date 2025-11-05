//! Test helpers for BPS checksum validation

use std::path::PathBuf;

pub fn test_rom_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_files/bps")
        .join(name)
}
