// Extra tests that extend `tests/ui/lint/invalid_value.rs` but will not work
// CHERI targets.
// ignore-purecap: additional tests

#![deny(invalid_value)]

use std::mem;

fn main() {
    unsafe {
        // Transmute-from-0
        let _val: &'static i32 = mem::transmute(0usize); //~ ERROR: does not permit zero-initialization
        let _val: &'static [i32] = mem::transmute((0usize, 0usize)); //~ ERROR: does not permit zero-initialization
    }
}
