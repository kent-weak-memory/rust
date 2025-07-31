// run-fail
// only-purecap: tests CHERI target features

// Check that reading outside of heap allocations causes CHERI exceptions.

use std::hint::black_box;

fn main() {
    let values: Box<[u32; 3]> = black_box(Box::new([0, 1, 2]));
    let pointer: *const u32 = &values[0];
    // Out of bounds, CHERI exception:
    unsafe { black_box(*(pointer.wrapping_add(4))) };
}
