//! Low-level I/O operations for IPS format
//!
//! Provides big-endian integer reading/writing utilities.

/// Read 24-bit big-endian unsigned integer
#[inline]
pub(super) fn read_u24_be(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32)
}

/// Read 16-bit big-endian unsigned integer
#[inline]
pub(super) fn read_u16_be(bytes: &[u8]) -> u16 {
    ((bytes[0] as u16) << 8) | (bytes[1] as u16)
}
