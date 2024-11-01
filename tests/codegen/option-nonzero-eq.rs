// compile-flags: -O -Zmerge-functions=disabled

#![crate_type = "lib"]

extern crate core;
use core::cmp::Ordering;
use core::num::{NonZeroU32, NonZeroI64};
use core::ptr::NonNull;

// See also tests/assembly/option-nonzero-eq.rs, for cases with `assume`s in the
// LLVM and thus don't optimize down clearly here, but do in assembly.

// CHECK-lABEL: @non_zero_eq
#[no_mangle]
pub fn non_zero_eq(l: Option<NonZeroU32>, r: Option<NonZeroU32>) -> bool {
    // CHECK: start:
    // CHECK-NEXT: icmp eq i32
    // CHECK-NEXT: ret i1
    l == r
}

// CHECK-lABEL: @non_zero_signed_eq
#[no_mangle]
pub fn non_zero_signed_eq(l: Option<NonZeroI64>, r: Option<NonZeroI64>) -> bool {
    // CHECK: start:
    // CHECK-NEXT: icmp eq i64
    // CHECK-NEXT: ret i1
    l == r
}

// CHECK-lABEL: @non_null_eq
#[no_mangle]
pub fn non_null_eq(l: Option<NonNull<u8>>, r: Option<NonNull<u8>>) -> bool {
    // CHECK: start:
    // TODO(seharris): Morello produces this extra code to explicitly check for
    //                 null, except it's a no-op?
    //                 I'm not really sure why this doesn't get removed, but I
    //                 also don't think it violates what this test seems to be
    //                 designed to verify?
    // CHERI: ptrtoint {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} %{{[0-9]+}} to i64
    // CHERI: icmp eq i64
    // CHERI: ptrtoint {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} %{{[0-9]+}} to i64
    // CHERI: icmp eq i64
    // CHERI: select i1 {{.*}} null,
    // CHERI: select i1 {{.*}} null,
    // CHECK-NEXT: icmp eq {{(i8( addrspace\(200\))?\*|ptr( addrspace\(200\))?)}}
    // CHECK-NEXT: ret i1
    l == r
}
