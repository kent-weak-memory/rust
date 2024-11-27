//
// no-system-llvm
// ignore-purecap TODO(seharris): something about Morello LLVM breaks this on both AArch64 and Morello.
// Presumably there are Rust specific changes in their LLVM fork, or we've turned off an optimisation this requires.
// The results are suboptimal, but don't appear to cause incorrect behavior.
// compile-flags: -O
#![crate_type = "lib"]

#[no_mangle]
pub fn alloc_test(data: u32) {
    // CHECK-LABEL: @alloc_test
    // CHECK-NEXT: start:
    // CHECK-NEXT: {{.*}} load volatile i8, {{i8( addrspace\(200\))?\*|ptr( addrspace\(200\))?}} @__rust_no_alloc_shim_is_unstable, align 1
    // CHECK-NEXT: ret void
    let x = Box::new(data);
    drop(x);
}
