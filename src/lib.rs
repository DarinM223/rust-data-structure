//! Reimplementations of common data structures in Rust

#![feature(rustc_private)]

extern crate arena;

pub mod arena_deque;
pub mod stack;
pub mod deque;
pub mod queue;
pub mod unsafe_queue;

pub use stack::Stack;
pub use deque::Deque;
pub use queue::Queue;
pub use unsafe_queue::List;
