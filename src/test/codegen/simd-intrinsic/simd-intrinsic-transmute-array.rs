//
// compile-flags: -C no-prepopulate-passes

#![crate_type = "lib"]

#![allow(non_camel_case_types)]
#![feature(repr_simd, platform_intrinsics, min_const_generics)]

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct S<const N: usize>([f32; N]);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct T([f32; 4]);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct U(f32, f32, f32, f32);

// CHECK-LABEL: @build_array_s
#[no_mangle]
pub fn build_array_s(x: [f32; 4]) -> S<4> {
    // NONCHERI: call void @llvm.memcpy.p0i8.p0i8.i{{[0-9]+}}(i8* {{.*}} %{{[0-9]+}}, i8* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    // CHERI: call void @llvm.memcpy.p200i8.p200i8.i{{[0-9]+}}(i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    // NONCHERI: call void @llvm.memcpy.p0i8.p0i8.i{{[0-9]+}}(i8* {{.*}} %{{[0-9]+}}, i8* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    // CHERI: call void @llvm.memcpy.p200i8.p200i8.i{{[0-9]+}}(i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    S::<4>(x)
}

// CHECK-LABEL: @build_array_t
#[no_mangle]
pub fn build_array_t(x: [f32; 4]) -> T {
    // NONCHERI: call void @llvm.memcpy.p0i8.p0i8.i{{[0-9]+}}(i8* {{.*}} %{{[0-9]+}}, i8* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    // CHERI: call void @llvm.memcpy.p200i8.p200i8.i{{[0-9]+}}(i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    // NONCHERI: call void @llvm.memcpy.p0i8.p0i8.i{{[0-9]+}}(i8* {{.*}} %{{[0-9]+}}, i8* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    // CHERI: call void @llvm.memcpy.p200i8.p200i8.i{{[0-9]+}}(i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    T(x)
}

// CHECK-LABEL: @build_array_u
#[no_mangle]
pub fn build_array_u(x: [f32; 4]) -> U {
    // NONCHERI: call void @llvm.memcpy.p0i8.p0i8.i{{[0-9]+}}(i8* {{.*}} %{{[0-9]+}}, i8* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    // CHERI: call void @llvm.memcpy.p200i8.p200i8.i{{[0-9]+}}(i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    // NONCHERI: call void @llvm.memcpy.p0i8.p0i8.i{{[0-9]+}}(i8* {{.*}} %{{[0-9]+}}, i8* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    // CHERI: call void @llvm.memcpy.p200i8.p200i8.i{{[0-9]+}}(i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i8 addrspace(200)* {{.*}} %{{[0-9]+}}, i{{[0-9]+}} 16, i1 false)
    unsafe { std::mem::transmute(x) }
}
