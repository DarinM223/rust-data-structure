//! A deque implementation using reference counting.
//!
//! It is completely safe because there are not unsafe
//! blocks but it is less performant than using
//! TypedArena + UnsafeCell or raw mutable pointers.

use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// Node for a doubly linked list
struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            data: data,
            next: None,
            prev: None,
        }))
    }
}

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>,
}

/// A deque implemented as a doubly linked list
impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_head = Node::new(data);

        match self.head.take() {
            Some(old_head) => {
                // Rust note: increment the reference count by one
                // (old_head has the "strong" reference to new_head)
                old_head.borrow_mut().prev = Some(new_head.clone());

                // Rust note: don't increment the reference count the other way
                // (or you will get into a cycle!)
                // (new_head has the "weak" reference to old_head)
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        };
    }

    pub fn push_back(&mut self, data: T) {
        let new_tail = Node::new(data);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        };
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            if let Some(new_head) = old_head.borrow_mut().next.take() {
                new_head.borrow_mut().prev.take();
                self.head = Some(new_head);
            } else {
                self.tail.take();
            }

            // Rc::try_unwrap unwraps a reference counted pointer only if
            // the reference count is 1
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            if let Some(new_tail) = old_tail.borrow_mut().prev.take() {
                new_tail.borrow_mut().next.take();
                self.tail = Some(new_tail);
            } else {
                self.head.take();
            }

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data
        })
    }
}

#[test]
fn test_deque() {
    let mut deque: Deque<i32> = Deque::new();
    deque.push_back(2);
    deque.push_back(3);
    assert_eq!(deque.pop_back(), Some(3));
    assert_eq!(deque.pop_back(), Some(2));

    deque.push_front(3);
    deque.push_front(2);
    assert_eq!(deque.pop_front(), Some(2));
    assert_eq!(deque.pop_front(), Some(3));

    deque.push_back(3);
    deque.push_back(2);
    assert_eq!(deque.pop_front(), Some(3));
    assert_eq!(deque.pop_front(), Some(2));
}
