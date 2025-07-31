// run-fail
// only-purecap: tests CHERI target features

// Check that reading outside of const allocations causes CHERI exceptions.

use std::hint::black_box;

const VALUES: [u32; 3] = [0, 1, 2];

fn main() {
    let pointer: *const u32 = black_box(&VALUES[0]);
    // Out of bounds, CHERI exception:
    unsafe { black_box(*(pointer.wrapping_add(4))) };
}
