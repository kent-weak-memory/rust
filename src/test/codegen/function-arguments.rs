// compile-flags: -O -C no-prepopulate-passes
//
// min-system-llvm-version: 12.0

#![crate_type = "lib"]
#![feature(rustc_attrs)]

pub struct S {
  _field: [i32; 8],
}

pub struct UnsafeInner {
  _field: std::cell::UnsafeCell<i16>,
}

// CHECK: zeroext i1 @boolean(i1 zeroext %x)
#[no_mangle]
pub fn boolean(x: bool) -> bool {
  x
}

// NONCHERI: @readonly_borrow(i32* noalias readonly align 4 dereferenceable(4) %_1)
// CHERI: @readonly_borrow(i32 addrspace(200)* noalias readonly align 4 dereferenceable(4) %_1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn readonly_borrow(_: &i32) {
}

// NONCHERI: @static_borrow(i32* noalias readonly align 4 dereferenceable(4) %_1)
// CHERI: @static_borrow(i32 addrspace(200)* noalias readonly align 4 dereferenceable(4) %_1)
// static borrow may be captured
#[no_mangle]
pub fn static_borrow(_: &'static i32) {
}

// NONCHERI: @named_borrow(i32* noalias readonly align 4 dereferenceable(4) %_1)
// CHERI: @named_borrow(i32 addrspace(200)* noalias readonly align 4 dereferenceable(4) %_1)
// borrow with named lifetime may be captured
#[no_mangle]
pub fn named_borrow<'r>(_: &'r i32) {
}

// NONCHERI: @unsafe_borrow(i16* align 2 dereferenceable(2) %_1)
// CHERI: @unsafe_borrow(i16 addrspace(200)* align 2 dereferenceable(2) %_1)
// unsafe interior means this isn't actually readonly and there may be aliases ...
#[no_mangle]
pub fn unsafe_borrow(_: &UnsafeInner) {
}

// NONCHERI: @mutable_unsafe_borrow(i16* noalias align 2 dereferenceable(2) %_1)
// CHERI: @mutable_unsafe_borrow(i16 addrspace(200)* noalias align 2 dereferenceable(2) %_1)
// ... unless this is a mutable borrow, those never alias
#[no_mangle]
pub fn mutable_unsafe_borrow(_: &mut UnsafeInner) {
}

// NONCHERI: @mutable_borrow(i32* noalias align 4 dereferenceable(4) %_1)
// CHERI: @mutable_borrow(i32 addrspace(200)* noalias align 4 dereferenceable(4) %_1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn mutable_borrow(_: &mut i32) {
}

// NONCHERI: @indirect_struct(%S* noalias nocapture dereferenceable(32) %_1)
// CHERI: @indirect_struct(%S addrspace(200)* noalias nocapture dereferenceable(32) %_1)
#[no_mangle]
pub fn indirect_struct(_: S) {
}

// NONCHERI: @borrowed_struct(%S* noalias readonly align 4 dereferenceable(32) %_1)
// CHERI: @borrowed_struct(%S addrspace(200)* noalias readonly align 4 dereferenceable(32) %_1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn borrowed_struct(_: &S) {
}

// `Box` can get deallocated during execution of the function, so it should
// not get `dereferenceable`.
// NONCHERI: noalias nonnull align 4 i32* @_box(i32* noalias nonnull align 4 %x)
// CHERI: noalias nonnull align 4 i32 addrspace(200)* @_box(i32 addrspace(200)* noalias nonnull align 4 %x)
#[no_mangle]
pub fn _box(x: Box<i32>) -> Box<i32> {
  x
}

// NONCHERI: @struct_return(%S* noalias nocapture sret(%S) dereferenceable(32){{( %0)?}})
// CHERI: @struct_return(%S addrspace(200)* noalias nocapture sret(%S) dereferenceable(32){{( %0)?}})
#[no_mangle]
pub fn struct_return() -> S {
  S {
    _field: [0, 0, 0, 0, 0, 0, 0, 0]
  }
}

// Hack to get the correct size for the length part in slices
// CHECK: @helper([[USIZE:i[0-9]+]] %_1)
#[no_mangle]
pub fn helper(_: usize) {
}

// NONCHERI: @slice([0 x i8]* noalias nonnull readonly align 1 %_1.0, [[USIZE]] %_1.1)
// CHERI: @slice([0 x i8] addrspace(200)* noalias nonnull readonly align 1 %_1.0, [[USIZE]] %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn slice(_: &[u8]) {
}

// NONCHERI: @mutable_slice([0 x i8]* noalias nonnull align 1 %_1.0, [[USIZE]] %_1.1)
// CHERI: @mutable_slice([0 x i8] addrspace(200)* noalias nonnull align 1 %_1.0, [[USIZE]] %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn mutable_slice(_: &mut [u8]) {
}

// NONCHERI: @unsafe_slice([0 x i16]* nonnull align 2 %_1.0, [[USIZE]] %_1.1)
// CHERI: @unsafe_slice([0 x i16] addrspace(200)* nonnull align 2 %_1.0, [[USIZE]] %_1.1)
// unsafe interior means this isn't actually readonly and there may be aliases ...
#[no_mangle]
pub fn unsafe_slice(_: &[UnsafeInner]) {
}

// NONCHERI: @str([0 x i8]* noalias nonnull readonly align 1 %_1.0, [[USIZE]] %_1.1)
// CHERI: @str([0 x i8] addrspace(200)* noalias nonnull readonly align 1 %_1.0, [[USIZE]] %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn str(_: &[u8]) {
}

// NONCHERI: @trait_borrow({}* nonnull align 1 %_1.0, [3 x [[USIZE]]]* noalias readonly align {{.*}} dereferenceable({{.*}}) %_1.1)
// CHERI: @trait_borrow({} addrspace(200)* nonnull align 1 %_1.0, [3 x [[USIZE]]] addrspace(200)* noalias readonly align {{.*}} dereferenceable({{.*}}) %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn trait_borrow(_: &Drop) {
}

// NONCHERI: @trait_box({}* noalias nonnull align 1{{( %0)?}}, [3 x [[USIZE]]]* noalias readonly align {{.*}} dereferenceable({{.*}}){{( %1)?}})
// CHERI: @trait_box({} addrspace(200)* noalias nonnull align 1{{( %0)?}}, [3 x [[USIZE]]] addrspace(200)* noalias readonly align {{.*}} dereferenceable({{.*}}){{( %1)?}})
#[no_mangle]
pub fn trait_box(_: Box<Drop>) {
}

// NONCHERI: { i8*, i8* } @trait_option(i8* noalias align 1 %x.0, i8* %x.1)
// CHERI: { i8 addrspace(200)*, i8 addrspace(200)* } @trait_option(i8 addrspace(200)* noalias align 1 %x.0, i8 addrspace(200)* %x.1)
#[no_mangle]
pub fn trait_option(x: Option<Box<Drop>>) -> Option<Box<Drop>> {
  x
}

// NONCHERI: { [0 x i16]*, [[USIZE]] } @return_slice([0 x i16]* noalias nonnull readonly align 2 %x.0, [[USIZE]] %x.1)
// CHERI: { [0 x i16] addrspace(200)*, [[USIZE]] } @return_slice([0 x i16] addrspace(200)* noalias nonnull readonly align 2 %x.0, [[USIZE]] %x.1)
#[no_mangle]
pub fn return_slice(x: &[u16]) -> &[u16] {
  x
}

// CHECK: { i16, i16 } @enum_id_1(i16 %x.0, i16 %x.1)
#[no_mangle]
pub fn enum_id_1(x: Option<Result<u16, u16>>) -> Option<Result<u16, u16>> {
  x
}

// CHECK: { i8, i8 } @enum_id_2(i1 zeroext %x.0, i8 %x.1)
#[no_mangle]
pub fn enum_id_2(x: Option<u8>) -> Option<u8> {
  x
}

// NONCHERI: noalias i8* @allocator()
// CHERI: noalias i8 addrspace(200)* @allocator()
#[no_mangle]
#[rustc_allocator]
pub fn allocator() -> *const i8 {
  std::ptr::null()
}
