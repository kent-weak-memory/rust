// compile-flags: -O -C no-prepopulate-passes -Zmir-opt-level=0

#![crate_type = "lib"]

// CHECK-LABEL: @test
#[no_mangle]
pub fn test() {
    let a = 0;
    &a; // keep variable in an alloca

// NONCHERI: [[S_a:%[0-9]+]] = bitcast i32* %a to i8*
// CHERI: [[S_a:%[0-9]+]] = bitcast i32 addrspace(200)* %a to i8 addrspace(200)*
// NONCHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, i8* [[S_a]])
// CHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, i8 addrspace(200)* [[S_a]])

    {
        let b = &Some(a);
        &b; // keep variable in an alloca

// NONCHERI: [[S_b:%[0-9]+]] = bitcast { i32, i32 }** %b to i8*
// CHERI: [[S_b:%[0-9]+]] = bitcast { i32, i32 } addrspace(200)* addrspace(200)* %b to i8 addrspace(200)*
// NONCHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, i8* [[S_b]])
// CHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, i8 addrspace(200)* [[S_b]])

// NONCHERI: [[S__4:%[0-9]+]] = bitcast { i32, i32 }* %_5 to i8*
// CHERI: [[S__4:%[0-9]+]] = bitcast { i32, i32 } addrspace(200)* %_5 to i8 addrspace(200)*
// NONCHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, i8* [[S__4]])
// CHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, i8 addrspace(200)* [[S__4]])

// NONCHERI: [[E__4:%[0-9]+]] = bitcast { i32, i32 }* %_5 to i8*
// CHERI: [[E__4:%[0-9]+]] = bitcast { i32, i32 } addrspace(200)* %_5 to i8 addrspace(200)*
// NONCHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, i8* [[E__4]])
// CHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, i8 addrspace(200)* [[E__4]])

// NONCHERI: [[E_b:%[0-9]+]] = bitcast { i32, i32 }** %b to i8*
// CHERI: [[E_b:%[0-9]+]] = bitcast { i32, i32 } addrspace(200)* addrspace(200)* %b to i8 addrspace(200)*
// NONCHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, i8* [[E_b]])
// CHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, i8 addrspace(200)* [[E_b]])
    }

    let c = 1;
    &c; // keep variable in an alloca

// NONCHERI: [[S_c:%[0-9]+]] = bitcast i32* %c to i8*
// CHERI: [[S_c:%[0-9]+]] = bitcast i32 addrspace(200)* %c to i8 addrspace(200)*
// NONCHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, i8* [[S_c]])
// CHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, i8 addrspace(200)* [[S_c]])

// NONCHERI: [[E_c:%[0-9]+]] = bitcast i32* %c to i8*
// CHERI: [[E_c:%[0-9]+]] = bitcast i32 addrspace(200)* %c to i8 addrspace(200)*
// NONCHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, i8* [[E_c]])
// CHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, i8 addrspace(200)* [[E_c]])

// NONCHERI: [[E_a:%[0-9]+]] = bitcast i32* %a to i8*
// CHERI: [[E_a:%[0-9]+]] = bitcast i32 addrspace(200)* %a to i8 addrspace(200)*
// NONCHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, i8* [[E_a]])
// CHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, i8 addrspace(200)* [[E_a]])
}
