// compile-flags: -C no-prepopulate-passes

#![crate_type="rlib"]

#[repr(align(16))]
pub struct S {
    arr: [u32; 4],
}

// CHECK-LABEL: @test1
// NONCHERI: store i32 0, i32* %{{.+}}, align 16
// CHERI: store i32 0, i32 addrspace(200)* %{{.+}}, align 16
// NONCHERI: store i32 1, i32* %{{.+}}, align 4
// CHERI: store i32 1, i32 addrspace(200)* %{{.+}}, align 4
// NONCHERI: store i32 2, i32* %{{.+}}, align 8
// CHERI: store i32 2, i32 addrspace(200)* %{{.+}}, align 8
// NONCHERI: store i32 3, i32* %{{.+}}, align 4
// CHERI: store i32 3, i32 addrspace(200)* %{{.+}}, align 4
#[no_mangle]
pub fn test1(s: &mut S) {
    s.arr[0] = 0;
    s.arr[1] = 1;
    s.arr[2] = 2;
    s.arr[3] = 3;
}

// CHECK-LABEL: @test2
// NONCHERI: store i32 4, i32* %{{.+}}, align 4
// CHERI: store i32 4, i32 addrspace(200)* %{{.+}}, align 4
#[allow(unconditional_panic)]
#[no_mangle]
pub fn test2(s: &mut S) {
    s.arr[usize::MAX / 4 + 1] = 4;
}

// CHECK-LABEL: @test3
// NONCHERI: store i32 5, i32* %{{.+}}, align 4
// CHERI: store i32 5, i32 addrspace(200)* %{{.+}}, align 4
#[no_mangle]
pub fn test3(s: &mut S, i: usize) {
    s.arr[i] = 5;
}

// CHECK-LABEL: @test4
// NONCHERI: store i32 6, i32* %{{.+}}, align 4
// CHERI: store i32 6, i32 addrspace(200)* %{{.+}}, align 4
#[no_mangle]
pub fn test4(s: &mut S) {
    s.arr = [6; 4];
}
