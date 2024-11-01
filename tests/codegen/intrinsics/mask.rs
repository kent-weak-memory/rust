// compile-flags: -Copt-level=0
#![crate_type = "lib"]
#![feature(core_intrinsics)]

// CHECK-LABEL: @mask_ptr
// CHECK-SAME: [[WORD:i[0-9]+]] %mask
#[no_mangle]
pub fn mask_ptr(ptr: *const u16, mask: usize) -> *const u16 {
    // CHECK: call
    // CHECK-SAME: @llvm.ptrmask.{{p[0-9]+|p[0-9]+i8}}.[[WORD]]({{ptr( addrspace\(200\))?|i8( addrspace\(200\))?\*}} {{%ptr|%1}}, [[WORD]] %mask)
    core::intrinsics::ptr_mask(ptr, mask)
}
