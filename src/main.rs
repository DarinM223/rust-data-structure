#![feature(alloc_system)]
extern crate alloc_system;

pub mod lru_cache;

use lru_cache::LRUCache;

fn main() {
    // Test LRUCache for memory leaks:

    let mut cache = LRUCache::new(3);
    cache.set(1, "1");
    cache.set(2, "2");
    cache.set(3, "3");
    cache.set(3, "3");

    // 3 is least recently used key
    assert_eq!(cache.get(3), Some("3"));
    assert_eq!(cache.get(2), Some("2"));
    assert_eq!(cache.get(1), Some("1"));
    assert_eq!(cache.get(2), Some("2"));

    // Set another value to evict least recently used key
    cache.set(4, "4");

    // Test that 3 got evicted and the others are still fine
    assert_eq!(cache.get(3), None);
    assert_eq!(cache.get(2), Some("2"));
    assert_eq!(cache.get(1), Some("1"));
    assert_eq!(cache.get(4), Some("4"));

    println!("Finished");
}
