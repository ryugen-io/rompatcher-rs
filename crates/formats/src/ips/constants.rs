//! IPS format constants and specifications

/// IPS header magic bytes: "PATCH"
pub(super) const HEADER: &[u8] = b"PATCH";

/// EOF marker as 24-bit integer (0x454F46)
pub(super) const EOF_MARKER: u32 = 0x454F46;

/// Maximum ROM size (16 MB due to 24-bit addressing)
pub const MAX_ROM_SIZE: usize = 16 * 1024 * 1024;

/// Maximum record size (64 KB due to 16-bit size field)
pub const MAX_RECORD_SIZE: usize = 65535;
