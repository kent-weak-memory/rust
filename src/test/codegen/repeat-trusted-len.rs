// compile-flags: -O
//

#![crate_type = "lib"]

use std::iter;

// CHECK-LABEL: @repeat_take_collect
#[no_mangle]
pub fn repeat_take_collect() -> Vec<u8> {
// NONCHERI: call void @llvm.memset.p0i8.i{{[0-9]+}}(i8* {{.*}}align 1{{.*}} %{{[0-9]+}}, i8 42, i{{[0-9]+}} 100000, i1 false)
// CHERI: call void @llvm.memset.p200i8.i{{[0-9]+}}(i8 addrspace(200)* {{.*}}align 1{{.*}} %{{[0-9]+}}, i8 42, i{{[0-9]+}} 100000, i1 false)
    iter::repeat(42).take(100000).collect()
}
