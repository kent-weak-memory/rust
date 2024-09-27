// compile-flags: -O

#![crate_type = "lib"]
#![feature(get_mut_unchecked, new_uninit)]

use std::sync::Arc;

// TODO(seharris): This seems to be broken for all targets by disabling SROA in
//                 LLVM. Review if we find a way to re-enable SROA.
// COM: // CHECK-LABEL: @new_from_array
// COM: #[no_mangle]
// COM: pub fn new_from_array(x: u64) -> Arc<[u64]> {
// COM:     // Ensure that we only generate one alloca for the array.
// COM:
// COM:     // CHECK: alloca
// COM:     // CHECK-SAME: [1000 x i64]
// COM:     // CHECK-NOT: alloca
// COM:     let array = [x; 1000];
// COM:     Arc::new(array)
// COM: }

// CHECK-LABEL: @new_uninit
#[no_mangle]
pub fn new_uninit(x: u64) -> Arc<[u64; 1000]> {
    // CHECK: call alloc::sync::arcinner_layout_for_value_layout
    // CHECK-NOT: call alloc::sync::arcinner_layout_for_value_layout
    let mut arc = Arc::new_uninit();
    unsafe { Arc::get_mut_unchecked(&mut arc) }.write([x; 1000]);
    unsafe { arc.assume_init() }
}

// CHECK-LABEL: @new_uninit_slice
#[no_mangle]
pub fn new_uninit_slice(x: u64) -> Arc<[u64]> {
    // CHECK: call alloc::sync::arcinner_layout_for_value_layout
    // CHECK-NOT: call alloc::sync::arcinner_layout_for_value_layout
    let mut arc = Arc::new_uninit_slice(1000);
    for elem in unsafe { Arc::get_mut_unchecked(&mut arc) } {
        elem.write(x);
    }
    unsafe { arc.assume_init() }
}
