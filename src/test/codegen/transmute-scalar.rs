// compile-flags: -O -C no-prepopulate-passes

#![crate_type = "lib"]

// FIXME(eddyb) all of these tests show memory stores and loads, even after a
// scalar `bitcast`, more special-casing is required to remove `alloca` usage.

// CHECK-LABEL: define{{.*}}i32 @f32_to_bits(float %x)
// CHECK: %2 = bitcast float %x to i32
// CHECK-NEXT: store i32 %2, i32{{( addrspace\(.*\))?}}* %0
// CHECK-NEXT: %3 = load i32, i32{{( addrspace\(.*\))?}}* %0
// CHECK: ret i32 %3
#[no_mangle]
pub fn f32_to_bits(x: f32) -> u32 {
    unsafe { std::mem::transmute(x) }
}

// CHECK-LABEL: define{{.*}}i8 @bool_to_byte(i1 zeroext %b)
// CHECK: %1 = zext i1 %b to i8
// CHECK-NEXT: store i8 %1, i8{{( addrspace\(.*\))?}}* %0
// CHECK-NEXT: %2 = load i8, i8{{( addrspace\(.*\))?}}* %0
// CHECK: ret i8 %2
#[no_mangle]
pub fn bool_to_byte(b: bool) -> u8 {
    unsafe { std::mem::transmute(b) }
}

// CHECK-LABEL: define{{.*}}zeroext i1 @byte_to_bool(i8 %byte)
// CHECK: %1 = trunc i8 %byte to i1
// CHECK-NEXT: %2 = zext i1 %1 to i8
// CHECK-NEXT: store i8 %2, i8{{( addrspace\(.*\))?}}* %0
// CHECK-NEXT: %3 = load i8, i8{{( addrspace\(.*\))?}}* %0
// CHECK-NEXT: %4 = trunc i8 %3 to i1
// CHECK: ret i1 %4
#[no_mangle]
pub unsafe fn byte_to_bool(byte: u8) -> bool {
    std::mem::transmute(byte)
}

// CHECK-LABEL: define{{.*}}i8{{( addrspace\(.*\))?}}* @ptr_to_ptr(i16{{( addrspace\(.*\))?}}* %p)
// CHECK: %2 = bitcast i16{{( addrspace\(.*\))?}}* %p to i8{{( addrspace\(.*\))?}}*
// CHECK-NEXT: store i8{{( addrspace\(.*\))?}}* %2, i8{{( addrspace\(.*\))?}}*{{( addrspace\(.*\))?}}* %0
// CHECK-NEXT: %3 = load i8{{( addrspace\(.*\))?}}*, i8{{( addrspace\(.*\))?}}*{{( addrspace\(.*\))?}}* %0
// CHECK: ret i8{{( addrspace\(.*\))?}}* %3
#[no_mangle]
pub fn ptr_to_ptr(p: *mut u16) -> *mut u8 {
    unsafe { std::mem::transmute(p) }
}

// Disabled because this doesn't work on CHERI and conflicts with some
// assumptions made for this fork of the Rust compiler.
// // HACK(eddyb) scalar `transmute`s between pointers and non-pointers are
// // currently not special-cased like other scalar `transmute`s, because
// // LLVM requires specifically `ptrtoint`/`inttoptr` instead of `bitcast`.
// //
// // Tests below show the non-special-cased behavior (with the possible
// // future special-cased instructions in the "NOTE(eddyb)" comments).
//
// // IGNORECHECK: define{{.*}}[[USIZE:i[0-9]+]] @ptr_to_int(i16{{( addrspace\(.*\))?}}* %p)
//
// // NOTE(eddyb) see above, the following two CHECK lines should ideally be this:
// //        %2 = ptrtoint i16{{( addrspace\(.*\))?}}* %p to [[USIZE]]
// //             store [[USIZE]] %2, [[USIZE]]{{( addrspace\(.*\))?}}* %0
// // IGNORECHECK: %2 = bitcast [[USIZE]]{{( addrspace\(.*\))?}}* %0 to i16{{( addrspace\(.*\))?}}*{{( addrspace\(.*\))?}}*
// // IGNORECHECK-NEXT: store i16{{( addrspace\(.*\))?}}* %p, i16{{( addrspace\(.*\))?}}*{{( addrspace\(.*\))?}}* %2
//
// // IGNORECHECK-NEXT: %3 = load [[USIZE]], [[USIZE]]{{( addrspace\(.*\))?}}* %0
// // IGNORECHECK: ret [[USIZE]] %3
// #[no_mangle]
// pub fn ptr_to_int(p: *mut u16) -> usize {
//     unsafe { std::mem::transmute(p) }
// }
//
// // IGNORECHECK: define{{.*}}i16* @int_to_ptr([[USIZE]] %i)
//
// // NOTE(eddyb) see above, the following two CHECK lines should ideally be this:
// //        %2 = inttoptr [[USIZE]] %i to i16{{( addrspace\(.*\))?}}*
// //             store i16{{( addrspace\(.*\))?}}* %2, i16{{( addrspace\(.*\))?}}*{{( addrspace\(.*\))?}}* %0
// // IGNORECHECK: %2 = bitcast i16{{( addrspace\(.*\))?}}*{{( addrspace\(.*\))?}}* %0 to [[USIZE]]{{( addrspace\(.*\))?}}*
// // IGNORECHECK-NEXT: store [[USIZE]] %i, [[USIZE]]{{( addrspace\(.*\))?}}* %2
//
// // IGNORECHECK-NEXT: %3 = load i16{{( addrspace\(.*\))?}}*, i16{{( addrspace\(.*\))?}}*{{( addrspace\(.*\))?}}* %0
// // IGNORECHECK: ret i16{{( addrspace\(.*\))?}}* %3
// #[no_mangle]
// pub fn int_to_ptr(i: usize) -> *mut u16 {
//     unsafe { std::mem::transmute(i) }
// }
