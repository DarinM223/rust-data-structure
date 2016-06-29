Data structure implementations in Rust
======================================

Since Rust cares about memory safety without using a garbage collector, it
can be difficult to implement common CS data structures like doubly-linked-lists
and graphs. Also even simpler data structures like trees and queues can be difficult
for people new to Rust.

This repository contains a work in progress collection of data structures in Rust
so that I can look back on them later if needed.

Building and testing
====================

To run the tests for the data structures, run:
```
cargo test
```

To check valgrind for memory leaks, run:
```
sh check_valgrind.sh
```
(You need to have valgrind installed and in the PATH)
