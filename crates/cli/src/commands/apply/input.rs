//! Input file loading with checksum display

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Load ROM file and display its checksum
pub fn load_rom_with_checksum(rom_path: &Path, verbose: u8) -> Result<Vec<u8>> {
    println!("Loading ROM: {}", rom_path.display());
    let rom_data = fs::read(rom_path).context("Failed to read ROM file")?;

    if verbose > 0 {
        println!("ROM size: {} bytes", rom_data.len());
    }

    #[cfg(feature = "validation")]
    {
        let crc = crate::utils::validation::compute_crc32(&rom_data);
        println!(
            "Input ROM CRC32: {}",
            crate::utils::validation::format_crc32(crc)
        );
    }

    Ok(rom_data)
}

/// Load patch file and display its checksum
pub fn load_patch_with_checksum(patch_path: &Path, verbose: u8) -> Result<Vec<u8>> {
    println!("Loading patch: {}", patch_path.display());
    let patch_data = fs::read(patch_path).context("Failed to read patch file")?;

    if verbose > 0 {
        println!("Patch size: {} bytes", patch_data.len());
    }

    #[cfg(feature = "validation")]
    {
        let crc = crate::utils::validation::compute_crc32(&patch_data);
        println!(
            "Patch CRC32: {}",
            crate::utils::validation::format_crc32(crc)
        );
    }

    Ok(patch_data)
}
