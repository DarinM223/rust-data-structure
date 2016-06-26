use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use std::ptr;

struct IDGenerator {
    curr_id: i32,
}

impl IDGenerator {
    pub fn new() -> IDGenerator {
        IDGenerator { curr_id: 0 }
    }

    pub fn id(&mut self) -> i32 {
        let id = self.curr_id;
        self.curr_id += 1;
        id
    }
}

struct Node<K, V> {
    id: i32,
    key: K,
    val: V,
    next: *mut Node<K, V>,
    prev: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, val: V, generator: &mut IDGenerator) -> Node<K, V> {
        Node {
            id: generator.id(),
            key: key,
            val: val,
            next: ptr::null_mut(),
            prev: None,
        }
    }
}

pub struct LRUCache<K: Eq + Hash, V> {
    pub capacity: i32,
    pub count: i32,
    id_generator: IDGenerator,
    page_map: HashMap<K, *mut Node<K, V>>,
    front: Option<Box<Node<K, V>>>,
    back: *mut Node<K, V>,
}

impl<K, V> LRUCache<K, V>
    where K: Eq + Hash + Copy
{
    pub fn new(capacity: i32) -> LRUCache<K, V> {
        LRUCache {
            capacity: capacity,
            count: 0,
            id_generator: IDGenerator::new(),
            page_map: HashMap::new(),
            front: None,
            back: ptr::null_mut(),
        }
    }

    fn remove(&mut self, n: *mut Node<K, V>) {
        unsafe {
            if let Some(ref mut prev) = *&mut (*n).prev {
                prev.next = (*n).next;
            } else {
                self.back = (*n).next;
            }

            if (*n).next.is_null() {
                self.front = (*n).prev.take();
            } else {
                (*(*n).next).prev = (*n).prev.take();
            }
        }
    }

    fn add_to_front(&mut self, n: *mut Node<K, V>) {
        unsafe {
            if self.back.is_null() {
                self.back = n;
            } else if let Some(ref mut prev) = self.front {
                prev.next = n;
            }

            (*n).next = ptr::null_mut();
            (*n).prev = self.front.take();

            self.front = Some(mem::transmute::<*mut Node<K, V>, Box<Node<K, V>>>(n));
        }
    }

    pub fn get(&mut self, k: K) -> Option<V> {
        let front_id = if let Some(ref node) = self.front {
            node.id
        } else {
            panic!("Front node does not exist")
        };

        let mut map = mem::replace(&mut self.page_map, HashMap::with_capacity(0));
        let result = map.get_mut(&k).map(|ref mut node| {
            unsafe {
                if (***node).id != front_id {
                    self.remove(**node);
                    self.add_to_front(**node);
                }

                mem::transmute::<*mut Node<K, V>, Box<Node<K, V>>>(**node).val
            }
        });
        mem::replace(&mut self.page_map, map);

        result
    }

    pub fn set(&mut self, k: K, v: V) {
        let mut map = mem::replace(&mut self.page_map, HashMap::with_capacity(0));
        let node_exists = if let Some(ref mut node) = map.get_mut(&k) {
            self.remove(**node);
            true
        } else {
            false
        };
        mem::replace(&mut self.page_map, map);

        if node_exists {
            let mut new_node = Box::new(Node::new(k, v, &mut self.id_generator));
            let new_node_ptr: *mut _ = &mut *new_node;

            self.page_map.insert(k, new_node_ptr);
            self.add_to_front(new_node_ptr);
        } else {
            if self.count == self.capacity {
                unsafe {
                    if let Some(ref mut node) = self.page_map.get_mut(&mut (*self.back).key) {
                        **node = ptr::null_mut();
                    }
                }

                let back = self.back;
                self.remove(back);
                self.count -= 1;
            }

            let mut new_front = Box::new(Node::new(k, v, &mut self.id_generator));
            let new_front_ptr: *mut _ = &mut *new_front;
            
            self.add_to_front(new_front_ptr);
            self.page_map.insert(k, new_front_ptr);
            self.count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_set_and_get() {
        let mut cache = LRUCache::new(10);
        cache.set(1, "hello");
        cache.set(2, "world");

        assert_eq!(cache.get(1), Some("hello"));
        assert_eq!(cache.get(2), Some("hello"));
    }
}
