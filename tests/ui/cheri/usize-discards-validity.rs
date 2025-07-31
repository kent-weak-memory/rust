// run-fail
// only-purecap: tests CHERI target features

// Check that usize doesn't have a validity tag and casts do actually cast.

use std::hint::black_box;

fn main() {
    let value: u32 = 314159;
    let pointer: *const u32 = black_box(&value);
    let address: usize = pointer as usize;
    let not_valid: *const u32 = address as *const u32;
    unsafe { black_box(*not_valid) }; // not valid, should cause a CHERI exception
}
