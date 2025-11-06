//! EBP format constants

/// EBP uses same magic as IPS (PATCH)
pub const MAGIC: &[u8; 5] = b"PATCH";
pub const MAGIC_SIZE: usize = 5;

/// EBP EOF marker (same as IPS)
pub const EOF_MARKER: &[u8; 3] = b"EOF";

/// JSON metadata markers
pub const JSON_START_MARKER: u8 = b'{';
pub const JSON_END_MARKER: u8 = b'}';
