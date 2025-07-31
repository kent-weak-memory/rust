// run-fail
// only-purecap: tests CHERI target features

// Check that reading outside of stack allocations causes CHERI exceptions.

use std::hint::black_box;

fn main() {
    // Make sure this doesn't get optimised into a constant or something.
    // We want this to definitely be on the stack.
    let values: [u32; 3] = black_box([0, 1, 2]);
    let pointer: *const u32 = &values[0];
    // Out of bounds, CHERI exception:
    unsafe { black_box(*(pointer.wrapping_add(3))) };
}
