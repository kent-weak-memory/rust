// compile-flags: -Zunleash-the-miri-inside-of-you
// only-purecap: see ptr_arith.rs for non-CHERI targets.
#![feature(core_intrinsics)]

// During CTFE, we prevent pointer-to-int casts.
// Pointer comparisons are prevented in the trait system.

static PTR_INT_CAST: () = {
    let x = &0 as *const _ as usize;
    //~^ ERROR could not evaluate static initializer
    //~| exposing pointers
    let _v = x == x;
};

// `transmute::<*const _, usize>()` doesn't work on CHERI due to the size
// difference between pointer and `usize`.
// This is still here just in case it covers any more arithmetic checks, but
// it likely doesn't.
static PTR_INT_CAST_1: () = unsafe {
    let x = &0 as *const _ as usize;
    //~^ ERROR could not evaluate static initializer
    //~| exposing pointers
    let _v = x + 0;
};

// I'd love to test pointer comparison, but that is not possible since
// their `PartialEq` impl is non-`const`.

fn main() {}
