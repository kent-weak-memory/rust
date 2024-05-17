// compile-flags: -O -Zmerge-functions=disabled
// ignore-debug (the extra assertions get in the way)

#![crate_type = "lib"]

use std::num::{NonZeroI16, NonZeroU32};

// #71602 reported a simple array comparison just generating a loop.
// This was originally fixed by ensuring it generates a single bcmp,
// but we now generate it as a load+icmp instead. `is_zero_slice` was
// tweaked to still test the case of comparison against a slice,
// and `is_zero_array` tests the new array-specific behaviour.
// The optimization was then extended to short slice-to-array comparisons,
// so the first test here now has a long slice to still get the bcmp.

// CHECK-LABEL: @is_zero_slice_long
#[no_mangle]
pub fn is_zero_slice_long(data: &[u8; 456]) -> bool {
    // CHECK: %[[BCMP:.+]] = tail call i32 @{{bcmp|memcmp}}({{.+}})
    // CHECK-NEXT: %[[EQ:.+]] = icmp eq i32 %[[BCMP]], 0
    // CHECK-NEXT: ret i1 %[[EQ]]
    &data[..] == [0; 456]
}

// CHECK-LABEL: @is_zero_slice_short
#[no_mangle]
pub fn is_zero_slice_short(data: &[u8; 4]) -> bool {
    // NONCHERI: %[[LOAD:.+]] = load i32, {{i32\*|ptr}} %{{.+}}, align 1
    // CHERI: %[[LOAD:.+]] = load i32, {{i32 addrspace\(200)\*|ptr addrspace\(200)}} %{{.+}}, align 1
    // CHECK-NEXT: %[[EQ:.+]] = icmp eq i32 %[[LOAD]], 0
    // CHECK-NEXT: ret i1 %[[EQ]]
    &data[..] == [0; 4]
}

// CHECK-LABEL: @is_zero_array
#[no_mangle]
pub fn is_zero_array(data: &[u8; 4]) -> bool {
    // NONCHERI: %[[LOAD:.+]] = load i32, {{i32\*|ptr}} %{{.+}}, align 1
    // CHERI: %[[LOAD:.+]] = load i32, {{i32 addrspace\(200)\*|ptr addrspace\(200)}} %{{.+}}, align 1
    // CHECK-NEXT: %[[EQ:.+]] = icmp eq i32 %[[LOAD]], 0
    // CHECK-NEXT: ret i1 %[[EQ]]
    *data == [0; 4]
}

// The following test the extra specializations to make sure that slice
// equality for non-byte types also just emit a `bcmp`, not a loop.

// CHECK-LABEL: @eq_slice_of_nested_u8(
// CHECK-SAME: [[USIZE:i16|i32|i64]] noundef %1
// CHECK-SAME: [[USIZE]] noundef %3
#[no_mangle]
fn eq_slice_of_nested_u8(x: &[[u8; 3]], y: &[[u8; 3]]) -> bool {
    // CHECK: icmp eq [[USIZE]] %1, %3
    // CHECK: %[[BYTES:.+]] = mul nsw [[USIZE]] %1, 3
    // NONCHERI: tail call{{( noundef)?}} i32 @{{bcmp|memcmp}}({{i8\*|ptr}}
    // CHERI: tail call{{( noundef)?}} i32 @{{bcmp|memcmp}}({{i8 addrspace\(200\)\*|ptr addrspace\(200\)}}
    // CHECK-SAME: , [[USIZE]]{{( noundef)?}} %[[BYTES]])
    x == y
}

// CHECK-LABEL: @eq_slice_of_i32(
// CHECK-SAME: [[USIZE:i16|i32|i64]] noundef %1
// CHECK-SAME: [[USIZE]] noundef %3
#[no_mangle]
fn eq_slice_of_i32(x: &[i32], y: &[i32]) -> bool {
    // CHECK: icmp eq [[USIZE]] %1, %3
    // CHECK: %[[BYTES:.+]] = shl nsw [[USIZE]] %1, 2
    // NONCHERI: tail call{{( noundef)?}} i32 @{{bcmp|memcmp}}({{i32\*|ptr}}
    // CHERI: tail call{{( noundef)?}} i32 @{{bcmp|memcmp}}({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}}
    // CHECK-SAME: , [[USIZE]]{{( noundef)?}} %[[BYTES]])
    x == y
}

// CHECK-LABEL: @eq_slice_of_nonzero(
// CHECK-SAME: [[USIZE:i16|i32|i64]] noundef %1
// CHECK-SAME: [[USIZE]] noundef %3
#[no_mangle]
fn eq_slice_of_nonzero(x: &[NonZeroU32], y: &[NonZeroU32]) -> bool {
    // CHECK: icmp eq [[USIZE]] %1, %3
    // CHECK: %[[BYTES:.+]] = shl nsw [[USIZE]] %1, 2
    // NONCHERI: tail call{{( noundef)?}} i32 @{{bcmp|memcmp}}({{i32\*|ptr}}
    // CHERI: tail call{{( noundef)?}} i32 @{{bcmp|memcmp}}({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}}
    // CHECK-SAME: , [[USIZE]]{{( noundef)?}} %[[BYTES]])
    x == y
}

// CHECK-LABEL: @eq_slice_of_option_of_nonzero(
// CHECK-SAME: [[USIZE:i16|i32|i64]] noundef %1
// CHECK-SAME: [[USIZE]] noundef %3
#[no_mangle]
fn eq_slice_of_option_of_nonzero(x: &[Option<NonZeroI16>], y: &[Option<NonZeroI16>]) -> bool {
    // CHECK: icmp eq [[USIZE]] %1, %3
    // CHECK: %[[BYTES:.+]] = shl nsw [[USIZE]] %1, 1
    // NONCHERI: tail call{{( noundef)?}} i32 @{{bcmp|memcmp}}({{i16\*|ptr}}
    // CHERI: tail call{{( noundef)?}} i32 @{{bcmp|memcmp}}({{i16 addrspace\(200\)\*|ptr addrspace\(200\)}}
    // CHECK-SAME: , [[USIZE]]{{( noundef)?}} %[[BYTES]])
    x == y
}
