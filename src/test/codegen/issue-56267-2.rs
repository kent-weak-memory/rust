// compile-flags: -C no-prepopulate-passes

#![crate_type="rlib"]

#[allow(dead_code)]
pub struct Foo<T> {
    foo: u64,
    bar: T,
}

// The load from bar.1 should have alignment 4. Not checking
// other loads here, as the alignment will be platform-dependent.

// NONCHERI: %{{.+}} = load i32, i32* %{{.+}}, align 4
// CHERI: %{{.+}} = load i32, i32 addrspace(200)* %{{.+}}, align 4
#[no_mangle]
pub fn test(x: Foo<(i32, i32)>) -> (i32, i32) {
    x.bar
}
