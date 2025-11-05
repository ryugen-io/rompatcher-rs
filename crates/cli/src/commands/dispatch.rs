//! Format dispatch logic for applying patches

use anyhow::Result;
use rom_patcher_core::{PatchFormat, PatchType};
use rom_patcher_formats::{bps::BpsPatcher, ips::IpsPatcher, ups::UpsPatcher};

/// Apply patch based on detected format
/// Always validates patch integrity (CRC32) before applying
pub fn apply_patch(
    rom: &mut Vec<u8>,
    patch: &[u8],
    patch_type: &PatchType,
) -> Result<()> {
    match patch_type {
        PatchType::Ips => {
            IpsPatcher::validate(patch)?;
            let patcher = IpsPatcher;
            patcher.apply(rom, patch)?;
        }
        PatchType::Bps => {
            BpsPatcher::validate(patch)?;
            let patcher = BpsPatcher;
            patcher.apply(rom, patch)?;
        }
        PatchType::Ups => {
            UpsPatcher::validate(patch)?;
            let patcher = UpsPatcher;
            patcher.apply(rom, patch)?;
        }
        _ => {
            anyhow::bail!("Format {} is not yet implemented", patch_type.name());
        }
    }

    Ok(())
}
