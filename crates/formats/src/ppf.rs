//! PPF (PlayStation Patch Format) format implementation
//!
//! PPF is primarily used for PlayStation 1 and 2 game patches.
//! Versions: PPF1, PPF2, PPF3

use rom_patcher_core::{PatchError, PatchFormat, PatchMetadata, PatchType, Result};

const PPF1_HEADER: &[u8] = b"PPF10";
const PPF2_HEADER: &[u8] = b"PPF20";
const PPF3_HEADER: &[u8] = b"PPF30";

/// PPF format patcher
pub struct PpfPatcher;

impl PatchFormat for PpfPatcher {
    fn can_handle(data: &[u8]) -> bool {
        if data.len() < 5 {
            return false;
        }

        &data[0..5] == PPF1_HEADER || &data[0..5] == PPF2_HEADER || &data[0..5] == PPF3_HEADER
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidFormat("Not a PPF patch".to_string()));
        }

        // TODO: Implement PPF patching logic
        let _ = rom;
        Err(PatchError::Other(
            "PPF implementation not yet complete".to_string(),
        ))
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidFormat("Not a PPF patch".to_string()));
        }

        let mut metadata = PatchMetadata::new(PatchType::Ppf);

        // Detect version
        if &patch[0..5] == PPF1_HEADER {
            metadata = metadata.with_extra("version".to_string(), "PPF1".to_string());
        } else if &patch[0..5] == PPF2_HEADER {
            metadata = metadata.with_extra("version".to_string(), "PPF2".to_string());
        } else if &patch[0..5] == PPF3_HEADER {
            metadata = metadata.with_extra("version".to_string(), "PPF3".to_string());
        }

        Ok(metadata)
    }

    fn validate(patch: &[u8]) -> Result<()> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidFormat("Not a PPF patch".to_string()));
        }

        Ok(())
    }
}
