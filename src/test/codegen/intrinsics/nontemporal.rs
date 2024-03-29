// compile-flags: -O

#![feature(core_intrinsics)]
#![crate_type = "lib"]

#[no_mangle]
pub fn a(a: &mut u32, b: u32) {
    // CHECK-LABEL: define{{.*}}void @a
    // NONCHERI: store i32 %b, i32* %a, align 4, !nontemporal
    // CHERI: store i32 %b, i32 addrspace(200)* %a, align 4, !nontemporal
    unsafe {
        std::intrinsics::nontemporal_store(a, b);
    }
}
