//! APS (Advance Patching System) format support

pub mod n64;

use rom_patcher_core::{PatchFormat, PatchMetadata, Result};

/// APS format patcher
pub struct ApsPatcher;

impl PatchFormat for ApsPatcher {
    fn can_handle(data: &[u8]) -> bool {
        n64::ApsN64Patcher::can_handle(data)
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        n64::ApsN64Patcher.apply(rom, patch)
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        n64::ApsN64Patcher::metadata(patch)
    }

    fn validate(patch: &[u8]) -> Result<()> {
        n64::ApsN64Patcher::validate(patch)
    }

    fn verify(rom: &[u8], patch: &[u8], target: Option<&[u8]>) -> Result<()> {
        n64::ApsN64Patcher::verify(rom, patch, target)
    }
}
