#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(cell_extras)]

/// Use `mod blah` to include a module in your crate/project
/// Use `use blah` to prefix a namespace so you can call it without the long namespace name
mod stack;
mod deque;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::clone::Clone;
use std::fmt::Display;
use std::fmt::Debug;
use std::mem;

fn main() {
    println!("Hello world!");
}