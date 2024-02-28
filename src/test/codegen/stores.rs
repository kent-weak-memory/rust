// compile-flags: -C no-prepopulate-passes
//

#![crate_type = "lib"]

pub struct Bytes {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
}

// CHECK-LABEL: small_array_alignment
// The array is stored as i32, but its alignment is lower, go with 1 byte to avoid target
// dependent alignment
#[no_mangle]
pub fn small_array_alignment(x: &mut [i8; 4], y: [i8; 4]) {
// CHECK: [[TMP:%.+]] = alloca i32
// CHECK: %y = alloca [4 x i8]
// NONCHERI: store i32 %0, i32* [[TMP]]
// CHERI: store i32 %0, i32 addrspace(200)* [[TMP]]
// NONCHERI: [[Y8:%[0-9]+]] = bitcast [4 x i8]* %y to i8*
// CHERI: [[Y8:%[0-9]+]] = bitcast [4 x i8] addrspace(200)* %y to i8 addrspace(200)*
// NONCHERI: [[TMP8:%[0-9]+]] = bitcast i32* [[TMP]] to i8*
// CHERI: [[TMP8:%[0-9]+]] = bitcast i32 addrspace(200)* [[TMP]] to i8 addrspace(200)*
// NONCHERI: call void @llvm.memcpy.{{.*}}(i8* align 1 [[Y8]], i8* align 4 [[TMP8]], i{{[0-9]+}} 4, i1 false)
// CHERI: call void @llvm.memcpy.{{.*}}(i8 addrspace(200)* align 1 [[Y8]], i8 addrspace(200)* align 4 [[TMP8]], i{{[0-9]+}} 4, i1 false)
    *x = y;
}

// CHECK-LABEL: small_struct_alignment
// The struct is stored as i32, but its alignment is lower, go with 1 byte to avoid target
// dependent alignment
#[no_mangle]
pub fn small_struct_alignment(x: &mut Bytes, y: Bytes) {
// CHECK: [[TMP:%.+]] = alloca i32
// CHECK: %y = alloca %Bytes
// NONCHERI: store i32 %0, i32* [[TMP]]
// CHERI: store i32 %0, i32 addrspace(200)* [[TMP]]
// NONCHERI: [[Y8:%[0-9]+]] = bitcast %Bytes* %y to i8*
// CHERI: [[Y8:%[0-9]+]] = bitcast %Bytes addrspace(200)* %y to i8 addrspace(200)*
// NONCHERI: [[TMP8:%[0-9]+]] = bitcast i32* [[TMP]] to i8*
// CHERI: [[TMP8:%[0-9]+]] = bitcast i32 addrspace(200)* [[TMP]] to i8 addrspace(200)*
// NONCHERI: call void @llvm.memcpy.{{.*}}(i8* align 1 [[Y8]], i8* align 4 [[TMP8]], i{{[0-9]+}} 4, i1 false)
// CHERI: call void @llvm.memcpy.{{.*}}(i8 addrspace(200)* align 1 [[Y8]], i8 addrspace(200)* align 4 [[TMP8]], i{{[0-9]+}} 4, i1 false)
    *x = y;
}
