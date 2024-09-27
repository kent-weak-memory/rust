// compile-flags: -O
//

#![crate_type = "lib"]

use std::iter;

// TODO(seharris): This seems to be broken for all targets by disabling SROA in
//                 LLVM. Review if we find a way to re-enable SROA.
//                 (Pattern doesn't match because target of memset isn't a
//                 simple `%[0-9]+` value, it's `%scevgep.i.i.i.i.i`)
// CHECK-LABEL: @repeat_take_collect
#[no_mangle]
pub fn repeat_take_collect() -> Vec<u8> {
// COM: // NONCHERI: call void @llvm.memset.{{.+}}({{i8\*|ptr}} {{.*}}align 1{{.*}} %{{[0-9]+}}, i8 42, i{{[0-9]+}} 100000, i1 false)
// COM: // CHERI: call void @llvm.memset.{{.+}}({{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} {{.*}}align 1{{.*}} %{{[0-9]+}}, i8 42, i{{[0-9]+}} 100000, i1 false)
    iter::repeat(42).take(100000).collect()
}

// CHECK-LABEL: @repeat_with_take_collect
#[no_mangle]
pub fn repeat_with_take_collect() -> Vec<u8> {
// COM: // NONCHERI: call void @llvm.memset.{{.+}}({{i8\*|ptr}} {{.*}}align 1{{.*}} %{{[0-9]+}}, i8 13, i{{[0-9]+}} 12345, i1 false)
// COM: // CHERI: call void @llvm.memset.{{.+}}({{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} {{.*}}align 1{{.*}} %{{[0-9]+}}, i8 13, i{{[0-9]+}} 12345, i1 false)
    iter::repeat_with(|| 13).take(12345).collect()
}
