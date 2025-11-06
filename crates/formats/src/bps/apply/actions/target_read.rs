//! TARGET_READ action handler

use super::ActionContext;
use rom_patcher_core::{PatchError, Result};

/// Execute TARGET_READ action
#[inline]
pub fn target_read(ctx: &mut ActionContext, length: usize) -> Result<()> {
    if *ctx.offset + length > ctx.patch.len() {
        return Err(PatchError::UnexpectedEof(
            "TargetRead exceeds patch bounds".to_string(),
        ));
    }

    ctx.target
        .extend_from_slice(&ctx.patch[*ctx.offset..*ctx.offset + length]);
    *ctx.offset += length;
    Ok(())
}
