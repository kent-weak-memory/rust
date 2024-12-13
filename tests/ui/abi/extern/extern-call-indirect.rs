// run-pass
// ignore-wasm32-bare no libc to test ffi with

#![feature(rustc_private)]

extern crate libc;

mod rustrt {
    extern crate libc;

    #[link(name = "rust_test_helpers", kind = "static")]
    extern "C" {
        pub fn rust_dbg_call(
            cb: extern "C" fn(libc::uintptr_t) -> libc::uintptr_t,
            data: libc::uintptr_t,
        ) -> libc::uintptr_t;
    }
}

extern "C" fn cb(data: libc::uintptr_t) -> libc::uintptr_t {
    // TODO(seharris): tidy this bodging if we figure out a better way to
    //                 implement uintptr_t.
    if data == 1 as libc::uintptr_t {
        data
    } else {
        (fact(data.wrapping_sub(1)) as usize * data as usize) as libc::uintptr_t
    }
}

fn fact(n: libc::uintptr_t) -> libc::uintptr_t {
    unsafe {
        println!("n = {}", n as usize);
        rustrt::rust_dbg_call(cb, n)
    }
}

pub fn main() {
    let result = fact(10 as libc::uintptr_t);
    println!("result = {}", result as usize);
    assert_eq!(result, 3628800 as libc::uintptr_t);
}
