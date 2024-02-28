// no-system-llvm
// compile-flags: -O
#![crate_type = "lib"]

type Demo = [u8; 3];

// CHECK-LABEL: @slice_iter_len_eq_zero
#[no_mangle]
pub fn slice_iter_len_eq_zero(y: std::slice::Iter<'_, Demo>) -> bool {
    // CHECK-NOT: sub
    // NONCHERI: %2 = icmp eq i8* %1, %0
    // Hopefully this is reasonable.
    // CHERI: %2 = icmp eq i64 %{{.*}}, %{{.*}}
    // CHECK: ret i1 %2
    y.len() == 0
}
