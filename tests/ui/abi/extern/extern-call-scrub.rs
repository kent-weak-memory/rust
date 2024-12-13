// run-pass
#![allow(unused_must_use)]
// This time we're testing repeatedly going up and down both stacks to
// make sure the stack pointers are maintained properly in both
// directions

// ignore-emscripten no threads support
#![feature(rustc_private)]

extern crate libc;
use std::thread;

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
        count(data.wrapping_sub(1))
            .wrapping_add(count(data.wrapping_sub(1)) as usize)
    }
}

fn count(n: libc::uintptr_t) -> libc::uintptr_t {
    unsafe {
        println!("n = {}", n as usize);
        rustrt::rust_dbg_call(cb, n)
    }
}

pub fn main() {
    // Make sure we're on a thread with small Rust stacks (main currently
    // has a large stack)
    thread::spawn(move || {
        let result = count(12 as libc::uintptr_t);
        println!("result = {}", result as usize);
        assert_eq!(result, 2048 as libc::uintptr_t);
    })
    .join();
}
