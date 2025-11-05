//! Test helpers for APS N64

pub fn make_header(output_size: u32) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS10");
    patch.push(0x01);
    patch.push(0x00);
    patch.extend_from_slice(&[0u8; 50]);
    patch.push(0x01);
    patch.extend_from_slice(b"TST");
    patch.extend_from_slice(&[0u8; 8]);
    patch.extend_from_slice(&[0u8; 5]);
    patch.extend_from_slice(&output_size.to_le_bytes());
    patch
}
