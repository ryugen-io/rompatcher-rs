//! APS (N64 APS) format implementation
//!
//! APS is primarily used for Nintendo 64 ROM patches.

use rom_patcher_core::{PatchError, PatchFormat, PatchMetadata, PatchType, Result};

const APS_HEADER: &[u8] = b"APS1";

/// APS format patcher
pub struct ApsPatcher;

impl PatchFormat for ApsPatcher {
    fn can_handle(data: &[u8]) -> bool {
        data.len() >= 4 && &data[0..4] == APS_HEADER
    }

    fn apply(&self, _rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidMagic {
                expected: APS_HEADER.to_vec(),
                actual: patch.get(0..4).unwrap_or(&[]).to_vec(),
            });
        }

        // TODO: Implement APS patching logic
        Err(PatchError::Other(
            "APS implementation not yet complete".to_string(),
        ))
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidFormat("Not an APS patch".to_string()));
        }

        Ok(PatchMetadata::new(PatchType::Aps))
    }

    fn validate(patch: &[u8]) -> Result<()> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidFormat("Not an APS patch".to_string()));
        }

        Ok(())
    }
}
