//! A stack implementation using a normal Box
//!
//! A stack has no cycles like doubly linked lists or graphs
//! so you can just use the default primitives like Box without
//! having to use reference counting or pointers.

use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

/// Node for a singly linked list
struct Node<T> {
    data: T,
    next: Link<T>,
}

/// Stack implementation using a singly linked list
pub struct Stack<T> {
    size: i32,
    head: Link<T>,
}

impl<T: Debug> Stack<T> {
    pub fn new() -> Self {
        Stack {
            size: 0,
            head: None,
        }
    }

    pub fn push(&mut self, data: T) {
        // Rust note: if you use take() on an Option
        // it sets the original option to None and
        // returns the value that was in the option.
        // This is useful for making sure there is only one
        // mutable reference to memory
        let new_head = Some(Box::new(Node {
            data: data,
            next: self.head.take(),
        }));

        self.head = new_head;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        // Rust note: if you set a reference to a property of a struct
        // you cannot set a reference to the actual struct but you can
        // set a reference to a different property of the same struct
        self.head.take().map(|old_head| {
            let old_head = *old_head;
            self.head = old_head.next;
            self.size -= 1;

            old_head.data
        })
    }

    // does the same thing as print_stack but inside the class
    // for notes about how it works, refer to print_stack
    pub fn print(&self) {
        let mut counter = &self.head;
        while let Some(ref n) = *counter {
            counter = &n.next;
        }
    }
}

/// print_stack_node prints a stack node link in a recursive manner
/// if a link is empty, return nothing, otherwise print the node and
/// recurse to the child
fn print_stack_node<T: Debug>(n: &Link<T>) {
    if let Some(ref node) = *n {
        print_stack_node(&node.next);
    }
}

/// print_stack_recur recursively prints a stack
pub fn print_stack_recur<T: Debug>(s: &Stack<T>) {
    print_stack_node(&s.head);
}

pub fn print_stack<T: Debug>(s: &Stack<T>) {
    // & means you can reference the variable, but you cannot mutate it
    // let mut means you can reassign it to other variables
    let mut counter = &s.head;
    while let Some(ref node) = *counter {
        counter = &node.next;
    }
}

#[test]
fn test_stack_push_and_pop() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);

    assert!(stack.size == 2);
    assert!(stack.pop() == Some(2));
    assert!(stack.pop() == Some(1));
    assert!(stack.pop() == None);
}
