// run-pass
// ignore-wasm32-bare no libc to test ffi with

// pretty-expanded FIXME #23616

mod a {
    extern "C" {
        pub fn free(x: *const u8);
    }
}

pub fn main() {
    unsafe {
        a::free(0_usize as *const u8);
    }
}
