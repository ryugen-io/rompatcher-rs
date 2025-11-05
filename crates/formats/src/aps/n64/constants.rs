//! APS N64 format constants

/// APS magic bytes "APS10"
pub const MAGIC: &[u8] = b"APS10";
pub const MAGIC_LEN: usize = 5;

/// Header type for N64
pub const HEADER_TYPE_N64: u8 = 0x01;

/// Record type: Simple (offset + length + data)
pub const RECORD_SIMPLE: u8 = 0x01;

/// Record type: RLE (offset + 0x00 + byte + length)
pub const RECORD_RLE: u8 = 0x00;

/// Description field length
pub const DESCRIPTION_LEN: usize = 50;

/// N64 cart ID offset in ROM
pub const N64_CART_ID_OFFSET: usize = 0x3C;
pub const N64_CART_ID_LEN: usize = 3;

/// N64 CRC offset in ROM
pub const N64_CRC_OFFSET: usize = 0x10;
pub const N64_CRC_LEN: usize = 8;

/// N64 header padding length
pub const N64_PAD_LEN: usize = 5;

/// Minimum patch file size (magic + header + output size)
pub const MIN_PATCH_SIZE: usize = MAGIC_LEN + 1 + 1 + DESCRIPTION_LEN + 4;

/// N64 header size (original format + cart ID + CRC + padding)
pub const N64_HEADER_SIZE: usize = 1 + N64_CART_ID_LEN + N64_CRC_LEN + N64_PAD_LEN;
