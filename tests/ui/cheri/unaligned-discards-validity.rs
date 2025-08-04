// run-fail
// only-purecap: tests CHERI target features

// Unaligned capabilities are cannot be valid due to the implementation of
// tag bits in the hardware.

#![feature(raw_ref_op)]

use std::hint::black_box;

#[repr(packed)]
struct A {
    padding: u8,
    invalid: *const u32,
    value: usize,
}

fn main() {
    let value: u32 = 314159;
    let data: A = black_box(A {
        padding: 0,
        invalid: &value,
        value: value as usize,
    });
    unsafe {
        assert_eq!(std::ptr::read_unaligned(&raw const data.value), value as usize);
        // Invalid capability, CHERI exception:
        assert_eq!(*std::ptr::read_unaligned(&raw const data.invalid), value);
    }
}
