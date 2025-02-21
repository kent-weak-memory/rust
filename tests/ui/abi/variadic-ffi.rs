// run-pass
// ignore-wasm32-bare no libc to test ffi with
#![feature(c_variadic)]

use std::ffi::VaList;

#[link(name = "rust_test_helpers", kind = "static")]
extern "C" {
    fn rust_interesting_average(_: u64, ...) -> f64;

    // FIXME: we need to disable this lint for `VaList`,
    // since it contains a `MaybeUninit<i32>` on the asmjs target,
    // and this type isn't FFI-safe. This is OK for now,
    // since the type is layout-compatible with `i32`.
    #[cfg_attr(target_arch = "asmjs", allow(improper_ctypes))]
    fn rust_valist_interesting_average(_: u64, _: VaList) -> f64;

    fn rust_aggregate_sum(count: u64, ...) -> f64;
    fn rust_aggregate_sum1(count: u64, ...) -> f64;
}

#[repr(C)]
struct Aggregate {
    a: u32,
    b: u64,
    c: f64,
}

#[repr(C)]
struct ListOfValues {
    int_count: u64,
    double_count: u64,
    int_values: *const u64,
    double_values: *const f64,
}

pub unsafe extern "C" fn test_valist_forward(n: u64, mut ap: ...) -> f64 {
    rust_valist_interesting_average(n, ap.as_va_list())
}

pub unsafe extern "C" fn test_va_copy(_: u64, mut ap: ...) {
    let mut ap2 = ap.clone();
    assert_eq!(rust_valist_interesting_average(2, ap2.as_va_list()) as i64, 30);

    // Advance one pair in the copy before checking
    let mut ap2 = ap.clone();
    let _ = ap2.arg::<u64>();
    let _ = ap2.arg::<f64>();
    assert_eq!(rust_valist_interesting_average(2, ap2.as_va_list()) as i64, 50);

    // Advance one pair in the original
    let _ = ap.arg::<u64>();
    let _ = ap.arg::<f64>();

    let mut ap2 = ap.clone();
    assert_eq!(rust_valist_interesting_average(2, ap2.as_va_list()) as i64, 50);

    let mut ap2 = ap.clone();
    let _ = ap2.arg::<u64>();
    let _ = ap2.arg::<f64>();
    assert_eq!(rust_valist_interesting_average(2, ap2.as_va_list()) as i64, 70);
}

pub fn main() {
    // Call without variadic arguments
    unsafe {
        assert!(rust_interesting_average(0).is_nan());
    }

    // Call with direct arguments
    unsafe {
        assert_eq!(rust_interesting_average(1, 10i64, 10.0f64) as i64, 20);
    }

    // Call with named arguments, variable number of them
    let (x1, x2, x3, x4) = (10i64, 10.0f64, 20i64, 20.0f64);
    unsafe {
        assert_eq!(rust_interesting_average(2, x1, x2, x3, x4) as i64, 30);
    }

    // Call with an aggregate that doesn't contain pointers.
    unsafe {
        let data0 = Aggregate{a: 123, b: 456, c: 78.9};
        let data1 = Aggregate{a: 987, b: 654, c: 3.21};
        assert_eq!(rust_aggregate_sum(2, data0, data1), 2302.11);
    }

    // Call with an aggregate that contains pointers.
    unsafe {
        let int_values0: [u64; 3] = [1, 2, 3];
        let int_values1: [u64; 3] = [4, 5, 6];
        let double_values0: [f64; 2] = [7.0, 8.9];
        let double_values1: [f64; 3] = [1.23, 45.6, 789.0];
        let data0 = ListOfValues {
            int_count: int_values0.len() as u64,
            double_count: double_values0.len() as u64,
            int_values: int_values0.as_ptr(),
            double_values: double_values0.as_ptr(),
        };
        let data1 = ListOfValues {
            int_count: int_values1.len() as u64,
            double_count: double_values1.len() as u64,
            int_values: int_values1.as_ptr(),
            double_values: double_values1.as_ptr(),
        };
        assert_eq!(rust_aggregate_sum1(2, data0, data1), 872.73);
    }

    // A function that takes a function pointer
    unsafe fn call(fp: unsafe extern "C" fn(u64, ...) -> f64) {
        let (x1, x2, x3, x4) = (10i64, 10.0f64, 20i64, 20.0f64);
        assert_eq!(fp(2, x1, x2, x3, x4) as i64, 30);
    }

    unsafe {
        call(rust_interesting_average);

        // Make a function pointer, pass indirectly
        let x: unsafe extern "C" fn(u64, ...) -> f64 = rust_interesting_average;
        call(x);
    }

    unsafe {
        assert_eq!(test_valist_forward(2, 10i64, 10f64, 20i64, 20f64) as i64, 30);
    }

    unsafe {
        test_va_copy(4, 10i64, 10f64, 20i64, 20f64, 30i64, 30f64, 40i64, 40f64);
    }
}
