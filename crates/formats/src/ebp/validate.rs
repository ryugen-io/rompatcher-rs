//! EBP validation

use super::constants::*;
use rom_patcher_core::Result;

/// Validate EBP patch format
/// EBP is IPS-compatible, so we delegate to IPS validation
pub fn validate(patch: &[u8]) -> Result<()> {
    // EBP uses same validation as IPS (magic + EOF + records)
    use crate::ips::IpsPatcher;
    use rom_patcher_core::PatchFormat;
    IpsPatcher::validate(patch)
}

/// Check if patch can be handled
pub fn can_handle(patch: &[u8]) -> bool {
    patch.len() >= MAGIC_SIZE && &patch[..MAGIC_SIZE] == MAGIC
}
