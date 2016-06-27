use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use std::ptr;

struct Node<K, V> {
    key: K,
    val: V,
    next: *mut Node<K, V>,
    prev: *mut Node<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, val: V) -> Node<K, V> {
        Node {
            key: key,
            val: val,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }
    }
}

pub struct LRUCache<K: Eq + Hash, V> {
    pub capacity: i32,
    pub count: i32,
    page_map: HashMap<K, *mut Node<K, V>>,
    front: *mut Node<K, V>,
    back: *mut Node<K, V>,
}

impl<K, V> LRUCache<K, V>
    where K: Eq + Hash + Copy,
          V: Clone
{
    pub fn new(capacity: i32) -> LRUCache<K, V> {
        LRUCache {
            capacity: capacity,
            count: 0,
            page_map: HashMap::new(),
            front: ptr::null_mut(),
            back: ptr::null_mut(),
        }
    }

    fn remove(&mut self, n: *mut Node<K, V>) {
        unsafe {
            if (*n).prev.is_null() {
                self.back = (*n).next;
            } else {
                (*(*n).prev).next = (*n).next;
            }

            if (*n).next.is_null() {
                self.front = (*n).prev;
            } else {
                (*(*n).next).prev = (*n).prev;
            }
        }
    }

    fn add_to_front(&mut self, n: *mut Node<K, V>) {
        unsafe {
            (*n).next = ptr::null_mut();
            (*n).prev = self.front;

            if self.back.is_null() {
                self.back = n;
            } else {
                (*self.front).next = n;
            }

            self.front = n;
        }
    }

    pub fn get(&mut self, k: K) -> Option<V> {
        if let Some(node) = self.page_map.remove(&k) {
            if node != self.front {
                self.remove(node);
                self.add_to_front(node);
            }
            self.page_map.insert(k, node);
            Some(unsafe { (*node).val.clone() })
        } else {
            None
        }
    }

    pub fn set(&mut self, k: K, v: V) {
        // Create the new front node
        let new_node = Box::new(Node::new(k, v));
        // For some reason let ptr: *mut _ = &mut *new_node doesn't
        // create a different pointer so we have to use mem::transmute.
        let new_node_ptr = unsafe { mem::transmute::<Box<Node<K, V>>, *mut Node<K, V>>(new_node) };

        if let Some(node) = self.page_map.remove(&k) {
            self.remove(node);
            self.page_map.insert(k, new_node_ptr);
            self.add_to_front(new_node_ptr);
        } else {
            if self.count == self.capacity {
                let back = self.back;
                unsafe {
                    self.page_map.remove(&(*back).key);
                }

                self.remove(back);
                self.count -= 1;
            }

            self.add_to_front(new_node_ptr);
            self.page_map.insert(k, new_node_ptr);
            self.count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut cache = LRUCache::new(10);

        cache.set(1, "hello");
        cache.set(2, "world");

        assert_eq!(cache.get(3), None);
        assert_eq!(cache.get(1), Some("hello"));
        assert_eq!(cache.get(2), Some("world"));
    }

    #[test]
    fn test_lru() {
        let mut cache = LRUCache::new(3);
        cache.set(1, "1");
        cache.set(2, "2");
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
    }
}
