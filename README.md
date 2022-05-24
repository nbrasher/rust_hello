# Rust Hello World

Implementation of a Least-Recently-Used cache in Rust, purely for the purposes of learning the language.

It turns out that doubly-linked lists are difficult to implement in Rust because of the requirements that each value have exactly one owner. This implementation uses a fixed length vec, with array indices in place of pointers.

### Usage
1. `cargo run` to run the code in `src/main.rs`
1. `cargo test` to run unit tests
