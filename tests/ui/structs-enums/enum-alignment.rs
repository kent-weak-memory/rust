// run-pass
#![allow(deprecated)]

use std::mem;

fn addr_of<T>(ptr: &T) -> usize {
    ptr as *const T as usize
}

fn is_aligned<T>(ptr: &T) -> bool {
    let addr: usize = addr_of(ptr);
    (addr % mem::min_align_of::<T>()) == 0
}

pub fn main() {
    let x = Some(0u64);
    match x {
        None => panic!(),
        Some(ref y) => assert!(is_aligned(y))
    }
}
