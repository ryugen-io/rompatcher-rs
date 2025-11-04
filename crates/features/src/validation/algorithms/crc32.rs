//! CRC32 checksum implementation

const CRC32_POLYNOMIAL: u32 = 0xEDB88320;

/// CRC32 hasher
pub(in crate::validation) struct Crc32Hasher {
    state: u32,
    table: [u32; 256],
}

impl Crc32Hasher {
    /// Create a new CRC32 hasher
    pub fn new() -> Self {
        let mut table = [0u32; 256];
        for (i, entry) in table.iter_mut().enumerate() {
            let mut crc = i as u32;
            for _ in 0..8 {
                crc = if crc & 1 == 1 {
                    (crc >> 1) ^ CRC32_POLYNOMIAL
                } else {
                    crc >> 1
                };
            }
            *entry = crc;
        }

        Self {
            state: 0xFFFFFFFF,
            table,
        }
    }

    /// Update hash with data
    pub fn update(&mut self, data: &[u8]) {
        for &byte in data {
            let index = ((self.state ^ byte as u32) & 0xFF) as usize;
            self.state = (self.state >> 8) ^ self.table[index];
        }
    }

    /// Finalize and return CRC32 checksum
    pub fn finalize(self) -> u32 {
        self.state ^ 0xFFFFFFFF
    }
}

impl Default for Crc32Hasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute CRC32 checksum of data
pub fn compute(data: &[u8]) -> u32 {
    let mut hasher = Crc32Hasher::new();
    hasher.update(data);
    hasher.finalize()
}
