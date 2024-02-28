// ignore-msvc
// ignore-wasm32-bare compiled with panic=abort by default

// compile-flags: -O -C no-prepopulate-passes

#![crate_type="lib"]

struct S;

impl Drop for S {
    fn drop(&mut self) {
    }
}

fn might_unwind() {
}

// CHECK-LABEL: @test
#[no_mangle]
pub fn test() {
    let _s = S;
    // Check that the personality slot alloca gets a lifetime start in each cleanup block, not just
    // in the first one.
    // NONCHERI: [[SLOT:%[0-9]+]] = alloca { i8*, i32 }
    // CHERI: [[SLOT:%[0-9]+]] = alloca { i8 addrspace(200)*, i32 }
    // CHECK-LABEL: cleanup:
    // NONCHERI: [[BITCAST:%[0-9]+]] = bitcast { i8*, i32 }* [[SLOT]] to i8*
    // CHERI: [[BITCAST:%[0-9]+]] = bitcast { i8 addrspace(200)*, i32 } addrspace(200)* [[SLOT]] to i8 addrspace(200)*
    // CHECK-NEXT: call void @llvm.lifetime.start.{{.*}}({{.*}}, i8{{( addrspace\(.*\))?}}* [[BITCAST]])
    // CHECK-LABEL: cleanup1:
    // NONCHERI: [[BITCAST1:%[0-9]+]] = bitcast { i8*, i32 }* [[SLOT]] to i8*
    // CHERI: [[BITCAST1:%[0-9]+]] = bitcast { i8 addrspace(200)*, i32 } addrspace(200)* [[SLOT]] to i8 addrspace(200)*
    // CHECK-NEXT: call void @llvm.lifetime.start.{{.*}}({{.*}}, i8{{( addrspace\(.*\))?}}* [[BITCAST1]])
    might_unwind();
    let _t = S;
    might_unwind();
}
