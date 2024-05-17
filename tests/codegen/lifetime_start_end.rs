// compile-flags: -O -C no-prepopulate-passes -Zmir-opt-level=0

#![crate_type = "lib"]

// CHECK-LABEL: @test
#[no_mangle]
pub fn test() {
    let a = 0u8;
    &a; // keep variable in an alloca

// NONCHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, {{i8\*|ptr}} %a)
// CHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} %a)

    {
        let b = &Some(a);
        &b; // keep variable in an alloca

// CHECK: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, {{.*}})

// CHECK: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, {{.*}})

// CHECK: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, {{.*}})

// CHECK: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, {{.*}})
    }

    let c = 1u8;
    &c; // keep variable in an alloca

// NONCHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, {{i8\*|ptr}} %c)
// CHERI: call void @llvm.lifetime.start{{.*}}(i{{[0-9 ]+}}, {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} %c)

// NONCHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, {{i8\*|ptr}} %c)
// CHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} %c)

// NONCHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, {{i8\*|ptr}} %a)
// CHERI: call void @llvm.lifetime.end{{.*}}(i{{[0-9 ]+}}, {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} %a)
}
