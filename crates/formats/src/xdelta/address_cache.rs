//! VCDIFF address cache implementation
//!
//! Handles the "near" and "same" address caching modes defined in RFC 3284.

pub struct AddressCache {
    near_size: usize,
    same_size: usize,
    near: Vec<u64>,
    same: Vec<u64>,
    next_near_slot: usize,
}

impl AddressCache {
    pub fn new(near_size: usize, same_size: usize) -> Self {
        Self {
            near_size,
            same_size,
            near: vec![0; near_size],
            same: vec![0; same_size * 256],
            next_near_slot: 0,
        }
    }

    pub fn reset(&mut self) {
        self.next_near_slot = 0;
        self.near.fill(0);
        self.same.fill(0);
    }

    pub fn update(&mut self, address: u64) {
        if self.near_size > 0 {
            self.near[self.next_near_slot] = address;
            self.next_near_slot = (self.next_near_slot + 1) % self.near_size;
        }

        if self.same_size > 0 {
            self.same[(address as usize) % (self.same_size * 256)] = address;
        }
    }

    pub fn get_near(&self, index: usize) -> u64 {
        self.near[index]
    }

    pub fn get_same(&self, index: usize) -> u64 {
        self.same[index]
    }
}

impl Default for AddressCache {
    fn default() -> Self {
        Self::new(4, 3) // Default per RFC 3284
    }
}
