// compile-flags: -C no-prepopulate-passes -Zmir-opt-level=0

#![crate_type = "lib"]

// Hack to get the correct size for the length part in slices
// CHECK: @helper([[USIZE:i[0-9]+]] %_1)
#[no_mangle]
pub fn helper(_: usize) {
}

// CHECK-LABEL: @ref_dst
#[no_mangle]
pub fn ref_dst(s: &[u8]) {
    // We used to generate an extra alloca and memcpy to ref the dst, so check that we copy
    // directly to the alloca for "x"
// NONCHERI: [[X0:%[0-9]+]] = getelementptr {{.*}} { [0 x i8]*, [[USIZE]] }* %x, i32 0, i32 0
// CHERI: [[X0:%[0-9]+]] = getelementptr {{.*}} { [0 x i8] addrspace(200)*, [[USIZE]] } addrspace(200)* %x, i32 0, i32 0
// NONCHERI: store [0 x i8]* %s.0, [0 x i8]** [[X0]]
// CHERI: store [0 x i8] addrspace(200)* %s.0, [0 x i8] addrspace(200)* addrspace(200)* [[X0]]
// NONCHERI: [[X1:%[0-9]+]] = getelementptr {{.*}} { [0 x i8]*, [[USIZE]] }* %x, i32 0, i32 1
// CHERI: [[X1:%[0-9]+]] = getelementptr {{.*}} { [0 x i8] addrspace(200)*, [[USIZE]] } addrspace(200)* %x, i32 0, i32 1
// NONCHERI: store [[USIZE]] %s.1, [[USIZE]]* [[X1]]
// CHERI: store [[USIZE]] %s.1, [[USIZE]] addrspace(200)* [[X1]]

    let x = &*s;
    &x; // keep variable in an alloca
}
