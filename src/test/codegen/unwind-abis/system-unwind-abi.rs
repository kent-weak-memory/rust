// compile-flags: -C opt-level=0

// Test that `nounwind` atributes are correctly applied to exported `system` and `system-unwind`
// extern functions. `system-unwind` functions MUST NOT have this attribute. We disable
// optimizations above to prevent LLVM from inferring the attribute.

#![crate_type = "lib"]
#![feature(c_unwind)]

// NONCHERI: @rust_item_that_cannot_unwind() unnamed_addr #0 {
// CHERI: @rust_item_that_cannot_unwind() unnamed_addr addrspace(200) #0 {
#[no_mangle]
pub extern "system" fn rust_item_that_cannot_unwind() {
}

// NONCHERI: @rust_item_that_can_unwind() unnamed_addr #1 {
// CHERI: @rust_item_that_can_unwind() unnamed_addr addrspace(200) #1 {
#[no_mangle]
pub extern "system-unwind" fn rust_item_that_can_unwind() {
}

// Now, make some assertions that the LLVM attributes for these functions are correct.  First, make
// sure that the first item is correctly marked with the `nounwind` attribute:
//
// CHECK: attributes #0 = { {{.*}}nounwind{{.*}} }
//
// Next, let's assert that the second item, which CAN unwind, does not have this attribute.
//
// CHECK: attributes #1 = {
// CHECK-NOT: nounwind
// CHECK: }
