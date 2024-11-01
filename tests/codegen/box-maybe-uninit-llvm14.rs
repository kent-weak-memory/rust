// compile-flags: -O
// ignore-purecap fails on Morello due to some weirdness with optimisations

// Once we're done with llvm 14 and earlier, this test can be deleted.

#![crate_type = "lib"]

use std::mem::MaybeUninit;

// Boxing a `MaybeUninit` value should not copy junk from the stack
#[no_mangle]
pub fn box_uninitialized() -> Box<MaybeUninit<usize>> {
    // CHECK-LABEL: @box_uninitialized
    // On Morello something about the extra steps needed to handle capabilities
    // seems to confuse LLVM's optimisations a bit, and things that would
    // normally get optimised away remain.
    // This results in a couple of what seem to be false positives.
    // The first is that this check matches `%storemerge.i.i.i`.
    // CHECK-NOT: store
    // The second false positive is that this check matches an `alloca` for
    // what seems to be a `Result` or something of a similar form.
    // On other targets the allocation gets optimised out and this works ok.
    // CHECK-NOT: alloca
    // CHECK-NOT: memcpy
    // CHECK-NOT: memset
    Box::new(MaybeUninit::uninit())
}

// https://github.com/rust-lang/rust/issues/58201
#[no_mangle]
pub fn box_uninitialized2() -> Box<MaybeUninit<[usize; 1024 * 1024]>> {
    // CHECK-LABEL: @box_uninitialized2
    // These two checks have the same problems on Morello as their counterparts
    // in the other test.
    // CHECK-NOT: store
    // CHECK-NOT: alloca
    // CHECK-NOT: memcpy
    // CHECK-NOT: memset
    Box::new(MaybeUninit::uninit())
}

// Hide the LLVM 15+ `allocalign` attribute in the declaration of __rust_alloc
// from the CHECK-NOT above. We don't check the attributes here because we can't rely
// on all of them being set until LLVM 15.
// CHECK: declare {{(dso_local )?}}noalias{{.*}} @__rust_alloc(i{{[0-9]+}} noundef, i{{[0-9]+.*}} noundef)
