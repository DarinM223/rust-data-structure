//! A deque implementation using a mutable
//! pointer container UnsafeCell with a TypedArena.
//!
//! It is safer than raw pointers because although the
//! UnsafeCell operations are unsafe, you won't have to
//! worry about use after free because the deque and all of
//! the nodes have the lifetime of the parent TypedArena.

use arena::TypedArena;
use std::cell::UnsafeCell;

pub struct Node<'a, T: 'a> {
    data: T,
    next: UnsafeCell<Option<&'a Node<'a, T>>>,
    prev: UnsafeCell<Option<&'a Node<'a, T>>>,
}

impl<'a, T: 'a> Node<'a, T> {
    pub fn new<'b>(data: T, arena: &'b TypedArena<Node<'b, T>>) -> &'b Node<'b, T> {
        arena.alloc(Node {
            data: data,
            next: UnsafeCell::new(None),
            prev: UnsafeCell::new(None),
        })
    }
}

pub struct Deque<'a, T: 'a> {
    arena: &'a TypedArena<Node<'a, T>>,
    front: UnsafeCell<Option<&'a Node<'a, T>>>,
    back: UnsafeCell<Option<&'a Node<'a, T>>>,
}

impl<'a, T: Clone> Deque<'a, T> {
    pub fn new(arena: &'a TypedArena<Node<'a, T>>) -> Deque<'a, T> {
        Deque {
            arena: arena,
            front: UnsafeCell::new(None),
            back: UnsafeCell::new(None),
        }
    }

    fn init_node(&mut self, data: T) {
        let init_node = Node::new(data, self.arena);

        // Set both the front and back pointers to the initial node
        unsafe {
            (*self.front.get()) = Some(init_node);
            (*self.back.get()) = Some(init_node);
        }
    }

    pub fn push_back(&mut self, data: T) {
        unsafe {
            if (*self.front.get()).is_none() {
                self.init_node(data);
            } else if let Some(old_back) = *self.back.get() {
                let new_back = Node::new(data, self.arena);

                // Set the new back pointer and old back pointer references
                (*new_back.next.get()) = Some(old_back);
                (*old_back.prev.get()) = Some(new_back);

                self.back = UnsafeCell::new(Some(new_back));
            }
        }
    }

    pub fn push_front(&mut self, data: T) {
        unsafe {
            if let Some(old_front) = *self.front.get() {
                let new_front = Node::new(data, self.arena);

                // Set the new front pointer and old front pointer references
                (*new_front.prev.get()) = Some(old_front);
                (*old_front.next.get()) = Some(new_front);

                self.front = UnsafeCell::new(Some(new_front));
            } else {
                self.init_node(data);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            if let Some(old_front) = *self.front.get() {
                (*self.front.get()) = *old_front.prev.get();
                return Some(old_front.data.clone());
            }
        }

        None
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            if let Some(old_back) = *self.back.get() {
                (*self.back.get()) = *old_back.next.get();
                return Some(old_back.data.clone());
            }
        }

        None
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (*self.front.get()).is_none() }
    }
}

#[cfg(test)]
mod tests {
    use arena::TypedArena;
    use super::*;

    #[test]
    fn test_empty() {
        let arena = TypedArena::new();

        let empty_deque = Deque::new(&arena);
        assert_eq!(empty_deque.is_empty(), true);

        let mut nonempty_deque = Deque::new(&arena);
        nonempty_deque.push_back(1);
        nonempty_deque.push_back(2);
        assert_eq!(nonempty_deque.is_empty(), false);
    }

    #[test]
    fn test_push_back() {
        let arena = TypedArena::new();
        let mut deque = Deque::new(&arena);

        deque.push_back(1);
        deque.push_back(2);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn test_push_front() {
        let arena = TypedArena::new();
        let mut deque = Deque::new(&arena);

        deque.push_front(1);
        deque.push_front(2);

        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn test_pop_back() {
        let arena = TypedArena::new();
        let mut deque = Deque::new(&arena);

        deque.push_back(1);
        deque.push_back(2);
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), None);
    }
}
