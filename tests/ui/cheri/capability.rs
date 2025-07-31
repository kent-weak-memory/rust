// run-pass
// only-purecap: tests CHERI target features

// Smoke tests for CHERI capabilities.

#![feature(strict_provenance)]

use std::hint::black_box;

const VALUE: u32 = 314159;

fn can_dereference() {
    let value: u32 = VALUE;
    let reference: &u32 = black_box(&value);
    assert_eq!(*reference, VALUE);
}

fn can_compare() {
    let values: [u32; 2] = [0, 1];
    let a: *const u32 = black_box(&values[0]);
    let b: *const u32 = black_box(&values[1]);
    let null: *const u32 = black_box(std::ptr::null());

    assert!(a != b);
    assert!(a == a);
    assert!(!a.is_null());
    assert!(null == null);
    assert!(a != null);
    assert!(null.is_null());
}

fn can_get_address() {
    let values: [u32; 2] = [0, 1];
    let a: *const u32 = black_box(&values[0]);
    let b: *const u32 = black_box(&values[1]);
    let null: *const u32 = black_box(std::ptr::null());
    
    assert_eq!(a as usize, a as usize);
    assert_eq!(a as usize+std::mem::size_of::<u32>(), b as usize);
    assert_eq!(a.addr(), a.addr());
    assert_eq!(a.addr()+std::mem::size_of::<u32>(), b.addr());
    assert_eq!(null.addr(), 0);
}

fn can_do_arithmetic() {
    let values: [u32; 2] = [0xdeadbeef, 0x11223344];
    let pointer: *const u32 = black_box(&values[0]);

    unsafe {
        assert_eq!(*pointer, values[0]);
        assert_eq!(*(pointer.wrapping_add(1)), values[1]);
    }
}

fn can_copy() {
    let value: u32 = VALUE;
    let a: &u32 = black_box(&value);
    let b: &u32 = a;
    assert_eq!(*a, value);
    assert_eq!(*b, value);
}

fn can_memcpy() {
    let values: [u32; 3] = [0, 1, 2];
    let references: [&u32; 3] = black_box([&values[0], &values[1], &values[2]]);
    let mut copies: [&u32; 3] = [&values[0], &values[0], &values[0]];
    copies.copy_from_slice(&references);
    assert_eq!(copies, references);
    assert_eq!(*copies[0], values[0]);
    assert_eq!(*copies[1], values[1]);
    assert_eq!(*copies[2], values[2]);
}

const CONST_REFERENCE: &'static u32 = &VALUE;
fn can_use_consts() {
    assert_eq!(*CONST_REFERENCE, VALUE);
}

pub fn main() {
    can_dereference();
    can_compare();
    can_get_address();
    can_do_arithmetic();
    can_copy();
    can_memcpy();
    can_use_consts();
}
