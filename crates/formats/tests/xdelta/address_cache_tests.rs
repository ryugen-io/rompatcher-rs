//! xdelta address cache tests

use rom_patcher_formats::xdelta::address_cache::AddressCache;

#[test]
fn test_cache_default() {
    let cache = AddressCache::default();
    assert_eq!(cache.near_size(), 4);
    // same_size is 3, total size 3*256 = 768
}

#[test]
fn test_cache_update() {
    let mut cache = AddressCache::default();
    
    // Update with address 100
    cache.update(100);
    
    // Near[0] should be 100
    assert_eq!(cache.get_near(0), 100);
    
    // Same[100 % (3*256)] should be 100
    // 100 % 768 = 100
    assert_eq!(cache.get_same(100), 100);
    
    // Update with address 200
    cache.update(200);
    // Near[1] should be 200
    assert_eq!(cache.get_near(1), 200);
    // Near[0] still 100
    assert_eq!(cache.get_near(0), 100);
}

#[test]
fn test_cache_reset() {
    let mut cache = AddressCache::default();
    cache.update(100);
    cache.reset();
    assert_eq!(cache.get_near(0), 0);
}
