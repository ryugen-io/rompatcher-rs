//! BPS (Beat Patching System) format implementation
//!
//! BPS was created by byuu (Near) for better error handling and efficiency.
//! Features:
//! - CRC32 checksums for source, target, and patch
//! - Variable-length integer encoding
//! - Multiple encoding methods (SourceRead, TargetRead, SourceCopy, TargetCopy)
//!
//! Format specification:
//! - Header: "BPS1" (4 bytes)
//! - Source size (variable length)
//! - Target size (variable length)
//! - Metadata size (variable length) + metadata
//! - Action commands
//! - Footer: source CRC32, target CRC32, patch CRC32

use rom_patcher_core::{PatchError, PatchFormat, PatchMetadata, PatchType, Result};

const BPS_HEADER: &[u8] = b"BPS1";

/// BPS format patcher
pub struct BpsPatcher;

impl PatchFormat for BpsPatcher {
    fn can_handle(data: &[u8]) -> bool {
        data.len() >= 4 && &data[0..4] == BPS_HEADER
    }

    fn apply(&self, _rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidMagic {
                expected: BPS_HEADER.to_vec(),
                actual: patch.get(0..4).unwrap_or(&[]).to_vec(),
            });
        }

        // TODO: Implement BPS patching logic
        Err(PatchError::Other(
            "BPS implementation not yet complete".to_string(),
        ))
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidFormat("Not a BPS patch".to_string()));
        }

        // TODO: Parse BPS metadata
        Ok(PatchMetadata::new(PatchType::Bps))
    }

    fn validate(patch: &[u8]) -> Result<()> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidFormat("Not a BPS patch".to_string()));
        }

        // TODO: Validate BPS checksums
        Ok(())
    }
}
