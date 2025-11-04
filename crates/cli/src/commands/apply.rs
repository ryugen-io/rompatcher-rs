//! Apply patch command with transactional safety

use anyhow::{Context, Result};
use rom_patcher_core::PatchFormat;
use rom_patcher_formats::{detect_format, ips::IpsPatcher};
use std::fs;
use std::path::{Path, PathBuf};

/// Apply a patch to a ROM file with transactional safety
///
/// Safety features:
/// - Clones ROM data before patching (rollback on error)
/// - Validates input != output paths
/// - Writes to temp file first, then atomic rename
/// - Always shows CRC32 checksums for verification
pub fn execute(rom_path: PathBuf, patch_path: PathBuf, output_path: Option<PathBuf>) -> Result<()> {
    // Generate default output path if not specified
    let output_path = match output_path {
        Some(path) => path,
        None => generate_default_output(&rom_path)?,
    };

    // Safety check: prevent overwriting input
    if rom_path == output_path {
        anyhow::bail!(
            "Input and output paths cannot be the same. Use a different output path to preserve the original ROM."
        );
    }

    println!("Loading ROM: {}", rom_path.display());
    let original_rom = fs::read(&rom_path).context("Failed to read ROM file")?;

    // Always show input ROM checksum
    #[cfg(feature = "validation")]
    {
        let input_crc = crate::utils::validation::compute_crc32(&original_rom);
        println!(
            "Input ROM CRC32: {}",
            crate::utils::validation::format_crc32(input_crc)
        );
    }

    println!("Loading patch: {}", patch_path.display());
    let patch_data = fs::read(&patch_path).context("Failed to read patch file")?;

    // Always show patch checksum
    #[cfg(feature = "validation")]
    {
        let patch_crc = crate::utils::validation::compute_crc32(&patch_data);
        println!(
            "Patch CRC32: {}",
            crate::utils::validation::format_crc32(patch_crc)
        );
    }

    // Auto-detect patch format
    let patch_type =
        detect_format(&patch_data).context("Could not detect patch format from file header")?;

    println!(
        "Detected format: {} ({})",
        patch_type.name(),
        patch_type.extension()
    );

    // Clone ROM data for transactional patching (rollback on error)
    let mut patched_rom = original_rom.clone();

    // Apply patch with format-specific handler
    apply_patch_by_type(&mut patched_rom, &patch_data, &patch_type)
        .context("Failed to apply patch")?;

    // Write to temp file first, then atomic rename
    let temp_path = output_path.with_extension("tmp");
    fs::write(&temp_path, &patched_rom).context("Failed to write temporary output file")?;

    fs::rename(&temp_path, &output_path).context("Failed to finalize output file")?;

    println!("Successfully patched: {}", output_path.display());
    println!(
        "ROM size: {} → {} bytes",
        original_rom.len(),
        patched_rom.len()
    );

    // Always show output checksum
    #[cfg(feature = "validation")]
    {
        let output_crc = crate::utils::validation::compute_crc32(&patched_rom);
        println!(
            "Output ROM CRC32: {}",
            crate::utils::validation::format_crc32(output_crc)
        );
    }

    Ok(())
}

/// Generate default output path: {rom_dir}/patched/{stem}.patched.{ext}
fn generate_default_output(rom_path: &Path) -> Result<PathBuf> {
    let rom_dir = rom_path
        .parent()
        .context("ROM file has no parent directory")?;

    let patched_dir = rom_dir.join("patched");
    fs::create_dir_all(&patched_dir).context("Failed to create patched/ directory")?;

    let file_stem = rom_path
        .file_stem()
        .and_then(|s| s.to_str())
        .context("ROM file has invalid filename")?;

    let extension = rom_path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let output_filename = if extension.is_empty() {
        format!("{}.patched", file_stem)
    } else {
        format!("{}.patched.{}", file_stem, extension)
    };

    Ok(patched_dir.join(output_filename))
}

/// Apply patch based on detected format
fn apply_patch_by_type(
    rom: &mut Vec<u8>,
    patch: &[u8],
    patch_type: &rom_patcher_core::PatchType,
) -> Result<()> {
    use rom_patcher_core::PatchType;

    match patch_type {
        PatchType::Ips => {
            let patcher = IpsPatcher;
            patcher.apply(rom, patch)?;
        }
        _ => {
            anyhow::bail!("Format {} is not yet implemented", patch_type.name());
        }
    }

    Ok(())
}
