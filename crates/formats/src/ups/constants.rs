//! UPS format constants

/// Magic bytes "UPS1"
pub const MAGIC: &[u8] = b"UPS1";
pub const MAGIC_SIZE: usize = 4;

/// Footer size: input CRC32 (4) + output CRC32 (4) + patch CRC32 (4)
pub const FOOTER_SIZE: usize = 12;
