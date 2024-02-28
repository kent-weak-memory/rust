// compile-flags: -C no-prepopulate-passes

#![crate_type = "lib"]

// Hack to get the correct size for the length part in slices
// CHECK: @helper([[USIZE:i[0-9]+]] %_1)
#[no_mangle]
pub fn helper(_: usize) {
}

// CHECK-LABEL: @no_op_slice_adjustment
#[no_mangle]
pub fn no_op_slice_adjustment(x: &[u8]) -> &[u8] {
    // We used to generate an extra alloca and memcpy for the block's trailing expression value, so
    // check that we copy directly to the return value slot
// NONCHERI: %0 = insertvalue { [0 x i8]*, [[USIZE]] } undef, [0 x i8]* %x.0, 0
// CHERI: %0 = insertvalue { [0 x i8] addrspace(200)*, [[USIZE]] } undef, [0 x i8] addrspace(200)* %x.0, 0
// NONCHERI: %1 = insertvalue { [0 x i8]*, [[USIZE]] } %0, [[USIZE]] %x.1, 1
// CHERI: %1 = insertvalue { [0 x i8] addrspace(200)*, [[USIZE]] } %0, [[USIZE]] %x.1, 1
// NONCHERI: ret { [0 x i8]*, [[USIZE]] } %1
// CHERI: ret { [0 x i8] addrspace(200)*, [[USIZE]] } %1
    { x }
}

// CHECK-LABEL: @no_op_slice_adjustment2
#[no_mangle]
pub fn no_op_slice_adjustment2(x: &[u8]) -> &[u8] {
    // We used to generate an extra alloca and memcpy for the function's return value, so check
    // that there's no memcpy (the slice is written to sret_slot element-wise)
// CHECK-NOT: call void @llvm.memcpy.
    no_op_slice_adjustment(x)
}
