// run-pass

// compile-flags: -Z thinlto -C codegen-units=8 -O
// ignore-emscripten can't inspect instructions on emscripten

// ignore-aarch64-unknown-freebsd-purecap
// CHERI BSD appears to seal function pointers, presumably as part of loading
// and relocation.
// It uses object type one, which means (if I understand correctly) that only
// way to unseal the capability is to branch to it.
// Because of this, there is no tidy way to read bytes via pointers to
// functions, and this test cannot be used on CHERI BSD targets.

// We want to assert here that ThinLTO will inline across codegen units. There's
// not really a great way to do that in general so we sort of hack around it by
// praying two functions go into separate codegen units and then assuming that
// if inlining *doesn't* happen the first byte of the functions will differ.

pub fn foo() -> u32 {
    bar::bar()
}

mod bar {
    pub fn bar() -> u32 {
        3
    }
}

fn main() {
    println!("{} {}", foo(), bar::bar());

    unsafe {
        let foo = foo as usize as *const u8;
        let bar = bar::bar as usize as *const u8;

        assert_eq!(*foo, *bar);
    }
}
