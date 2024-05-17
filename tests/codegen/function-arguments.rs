// compile-flags: -O -C no-prepopulate-passes

#![crate_type = "lib"]
#![feature(dyn_star)]

use std::mem::MaybeUninit;
use std::num::NonZeroU64;
use std::marker::PhantomPinned;
use std::ptr::NonNull;

pub struct S {
  _field: [i32; 8],
}

pub struct UnsafeInner {
  _field: std::cell::UnsafeCell<i16>,
}

pub struct NotUnpin {
  _field: i32,
  _marker: PhantomPinned,
}

pub enum MyBool {
  True,
  False,
}

// CHECK: noundef zeroext i1 @boolean(i1 noundef zeroext %x)
#[no_mangle]
pub fn boolean(x: bool) -> bool {
  x
}

// CHECK: i8 @maybeuninit_boolean(i8 %x)
#[no_mangle]
pub fn maybeuninit_boolean(x: MaybeUninit<bool>) -> MaybeUninit<bool> {
  x
}

// CHECK: noundef zeroext i1 @enum_bool(i1 noundef zeroext %x)
#[no_mangle]
pub fn enum_bool(x: MyBool) -> MyBool {
  x
}

// CHECK: i8 @maybeuninit_enum_bool(i8 %x)
#[no_mangle]
pub fn maybeuninit_enum_bool(x: MaybeUninit<MyBool>) -> MaybeUninit<MyBool> {
  x
}

// CHECK: noundef i32 @char(i32 noundef %x)
#[no_mangle]
pub fn char(x: char) -> char {
  x
}

// CHECK: i32 @maybeuninit_char(i32 %x)
#[no_mangle]
pub fn maybeuninit_char(x: MaybeUninit<char>) -> MaybeUninit<char> {
  x
}

// CHECK: noundef i64 @int(i64 noundef %x)
#[no_mangle]
pub fn int(x: u64) -> u64 {
  x
}

// CHECK: noundef i64 @nonzero_int(i64 noundef %x)
#[no_mangle]
pub fn nonzero_int(x: NonZeroU64) -> NonZeroU64 {
  x
}

// CHECK: noundef i64 @option_nonzero_int(i64 noundef %x)
#[no_mangle]
pub fn option_nonzero_int(x: Option<NonZeroU64>) -> Option<NonZeroU64> {
  x
}

// NONCHERI: @readonly_borrow({{i32\*|ptr}} noalias noundef readonly align 4 dereferenceable(4) %_1)
// CHERI: @readonly_borrow({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef readonly align 4 dereferenceable(4) %_1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn readonly_borrow(_: &i32) {
}

// NONCHERI: noundef align 4 dereferenceable(4) {{i32\*|ptr}} @readonly_borrow_ret()
// CHERI: noundef align 4 dereferenceable(4) {{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} @readonly_borrow_ret()
#[no_mangle]
pub fn readonly_borrow_ret() -> &'static i32 {
  loop {}
}

// NONCHERI: @static_borrow({{i32\*|ptr}} noalias noundef readonly align 4 dereferenceable(4) %_1)
// CHERI: @static_borrow({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef readonly align 4 dereferenceable(4) %_1)
// static borrow may be captured
#[no_mangle]
pub fn static_borrow(_: &'static i32) {
}

// NONCHERI: @named_borrow({{i32\*|ptr}} noalias noundef readonly align 4 dereferenceable(4) %_1)
// CHERI: @named_borrow({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef readonly align 4 dereferenceable(4) %_1)
// borrow with named lifetime may be captured
#[no_mangle]
pub fn named_borrow<'r>(_: &'r i32) {
}

// NONCHERI: @unsafe_borrow({{i16\*|ptr}} noundef nonnull align 2 %_1)
// CHERI: @unsafe_borrow({{i16 addrspace\(200\)\*|ptr addrspace\(200\)}} noundef nonnull align 2 %_1)
// unsafe interior means this isn't actually readonly and there may be aliases ...
#[no_mangle]
pub fn unsafe_borrow(_: &UnsafeInner) {
}

// NONCHERI: @mutable_unsafe_borrow({{i16\*|ptr}} noalias noundef align 2 dereferenceable(2) %_1)
// CHERI: @mutable_unsafe_borrow({{i16 addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef align 2 dereferenceable(2) %_1)
// ... unless this is a mutable borrow, those never alias
#[no_mangle]
pub fn mutable_unsafe_borrow(_: &mut UnsafeInner) {
}

// NONCHERI: @mutable_borrow({{i32\*|ptr}} noalias noundef align 4 dereferenceable(4) %_1)
// CHERI: @mutable_borrow({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef align 4 dereferenceable(4) %_1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn mutable_borrow(_: &mut i32) {
}

// NONCHERI: noundef align 4 dereferenceable(4) {{i32\*|ptr}} @mutable_borrow_ret()
// CHERI: noundef align 4 dereferenceable(4) {{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} @mutable_borrow_ret()
#[no_mangle]
pub fn mutable_borrow_ret() -> &'static mut i32 {
  loop {}
}

#[no_mangle]
// NONCHERI: @mutable_notunpin_borrow({{i32\*|ptr}} noundef nonnull align 4 %_1)
// CHERI: @mutable_notunpin_borrow({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noundef nonnull align 4 %_1)
// This one is *not* `noalias` because it might be self-referential.
// It is also not `dereferenceable` due to
// <https://github.com/rust-lang/unsafe-code-guidelines/issues/381>.
pub fn mutable_notunpin_borrow(_: &mut NotUnpin) {
}

// NONCHERI: @notunpin_borrow({{i32\*|ptr}} noalias noundef readonly align 4 dereferenceable(4) %_1)
// CHERI: @notunpin_borrow({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef readonly align 4 dereferenceable(4) %_1)
// But `&NotUnpin` behaves perfectly normal.
#[no_mangle]
pub fn notunpin_borrow(_: &NotUnpin) {
}

// NONCHERI: @indirect_struct({{%S\*|ptr}} noalias nocapture noundef readonly dereferenceable(32) %_1)
// CHERI: @indirect_struct({{%S addrspace\(200\)\*|ptr}} noalias nocapture noundef readonly dereferenceable(32) %_1)
#[no_mangle]
pub fn indirect_struct(_: S) {
}

// NONCHERI: @borrowed_struct({{%S\*|ptr}} noalias noundef readonly align 4 dereferenceable(32) %_1)
// CHERI: @borrowed_struct({{%S addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef readonly align 4 dereferenceable(32) %_1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn borrowed_struct(_: &S) {
}

// NONCHERI: @option_borrow({{i32\*|ptr}} noalias noundef readonly align 4 dereferenceable_or_null(4) %x)
// CHERI: @option_borrow({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef readonly align 4 dereferenceable_or_null(4) %x)
#[no_mangle]
pub fn option_borrow(x: Option<&i32>) {
}

// NONCHERI: @option_borrow_mut({{i32\*|ptr}} noalias noundef align 4 dereferenceable_or_null(4) %x)
// CHERI: @option_borrow_mut({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef align 4 dereferenceable_or_null(4) %x)
#[no_mangle]
pub fn option_borrow_mut(x: Option<&mut i32>) {
}

// NONCHERI: @raw_struct({{%S\*|ptr}} noundef %_1)
// CHERI: @raw_struct({{%S addrspace\(200\)\*|ptr addrspace\(200\)}} noundef %_1)
#[no_mangle]
pub fn raw_struct(_: *const S) {
}

// NONCHERI: @raw_option_nonnull_struct({{i32\*|ptr}} noundef %_1)
// CHERI: @raw_option_nonnull_struct({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noundef %_1)
#[no_mangle]
pub fn raw_option_nonnull_struct(_: Option<NonNull<S>>) {
}


// `Box` can get deallocated during execution of the function, so it should
// not get `dereferenceable`.
// NONCHERI: noundef nonnull align 4 {{i32\*|ptr}} @_box({{i32\*|ptr}} noalias noundef nonnull align 4 %x)
// CHERI: noundef nonnull align 4 {{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} @_box({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef nonnull align 4 %x)
#[no_mangle]
pub fn _box(x: Box<i32>) -> Box<i32> {
  x
}

// NONCHERI: noundef nonnull align 4 {{i32\*|ptr}} @notunpin_box({{i32\*|ptr}} noundef nonnull align 4 %x)
// CHERI: noundef nonnull align 4 {{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} @notunpin_box({{i32 addrspace\(200\)\*|ptr addrspace\(200\)}} noundef nonnull align 4 %x)
#[no_mangle]
pub fn notunpin_box(x: Box<NotUnpin>) -> Box<NotUnpin> {
  x
}

// NONCHERI: @struct_return({{%S\*|ptr}} noalias nocapture noundef sret(%S) dereferenceable(32){{( %0)?}})
// CHERI: @struct_return({{%S addrspace\(200\)\*|ptr addrspace\(200\)}} noalias nocapture noundef sret(%S) dereferenceable(32){{( %0)?}})
#[no_mangle]
pub fn struct_return() -> S {
  S {
    _field: [0, 0, 0, 0, 0, 0, 0, 0]
  }
}

// Hack to get the correct size for the length part in slices
// CHECK: @helper([[USIZE:i[0-9]+]] noundef %_1)
#[no_mangle]
pub fn helper(_: usize) {
}

// NONCHERI: @slice({{\[0 x i8\]\*|ptr}} noalias noundef nonnull readonly align 1 %_1.0, [[USIZE]] noundef %_1.1)
// CHERI: @slice({{\[0 x i8\] addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef nonnull readonly align 1 %_1.0, [[USIZE]] noundef %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn slice(_: &[u8]) {
}

// NONCHERI: @mutable_slice({{\[0 x i8\]\*|ptr}} noalias noundef nonnull align 1 %_1.0, [[USIZE]] noundef %_1.1)
// CHERI: @mutable_slice({{\[0 x i8\] addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef nonnull align 1 %_1.0, [[USIZE]] noundef %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn mutable_slice(_: &mut [u8]) {
}

// NONCHERI: @unsafe_slice({{\[0 x i16\]\*|ptr}} noundef nonnull align 2 %_1.0, [[USIZE]] noundef %_1.1)
// CHERI: @unsafe_slice({{\[0 x i16\] addrspace\(200\)\*|ptr addrspace\(200\)}} noundef nonnull align 2 %_1.0, [[USIZE]] noundef %_1.1)
// unsafe interior means this isn't actually readonly and there may be aliases ...
#[no_mangle]
pub fn unsafe_slice(_: &[UnsafeInner]) {
}

// NONCHERI: @raw_slice({{\[0 x i8\]\*|ptr}} noundef %_1.0, [[USIZE]] noundef %_1.1)
// CHERI: @raw_slice({{\[0 x i8\] addrspace\(200\)\*|ptr addrspace\(200\)}} noundef %_1.0, [[USIZE]] noundef %_1.1)
#[no_mangle]
pub fn raw_slice(_: *const [u8]) {
}

// NONCHERI: @str({{\[0 x i8\]\*|ptr}} noalias noundef nonnull readonly align 1 %_1.0, [[USIZE]] noundef %_1.1)
// CHERI: @str({{\[0 x i8\] addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef nonnull readonly align 1 %_1.0, [[USIZE]] noundef %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn str(_: &[u8]) {
}

// NONCHERI: @trait_borrow({{\{\}\*|ptr}} noundef nonnull align 1 %_1.0, {{.+}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %_1.1)
// CHERI: @trait_borrow({{\{\} addrspace\(200\)\*|ptr addrspace\(200\)}} noundef nonnull align 1 %_1.0, {{.+}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn trait_borrow(_: &dyn Drop) {
}

// NONCHERI: @option_trait_borrow({{i8\*|ptr}} noundef align 1 %x.0, {{i8\*|ptr}} %x.1)
// CHERI: @option_trait_borrow({{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} noundef align 1 %x.0, {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} %x.1)
#[no_mangle]
pub fn option_trait_borrow(x: Option<&dyn Drop>) {
}

// NONCHERI: @option_trait_borrow_mut({{i8\*|ptr}} noundef align 1 %x.0, {{i8\*|ptr}} %x.1)
// CHERI: @option_trait_borrow_mut({{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} noundef align 1 %x.0, {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} %x.1)
#[no_mangle]
pub fn option_trait_borrow_mut(x: Option<&mut dyn Drop>) {
}

// NONCHERI: @trait_raw({{\{\}\*|ptr}} noundef %_1.0, {{.+}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %_1.1)
// CHERI: @trait_raw({{\{\} addrspace\(200\)\*|ptr addrspace\(200\)}} noundef %_1.0, {{.+}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %_1.1)
#[no_mangle]
pub fn trait_raw(_: *const dyn Drop) {
}

// NONCHERI: @trait_box({{\{\}\*|ptr}} noalias noundef nonnull align 1{{( %0)?}}, {{.+}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}){{( %1)?}})
// CHERI: @trait_box({{\{\} addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef nonnull align 1{{( %0)?}}, {{.+}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}){{( %1)?}})
#[no_mangle]
pub fn trait_box(_: Box<dyn Drop + Unpin>) {
}

// NONCHERI: { {{i8\*|ptr}}, {{i8\*|ptr}} } @trait_option({{i8\*|ptr}} noalias noundef align 1 %x.0, {{i8\*|ptr}} %x.1)
// CHERI: { {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}}, {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} } @trait_option({{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef align 1 %x.0, {{i8 addrspace\(200\)\*|ptr addrspace\(200\)}} %x.1)
#[no_mangle]
pub fn trait_option(x: Option<Box<dyn Drop + Unpin>>) -> Option<Box<dyn Drop + Unpin>> {
  x
}

// NONCHERI: { {{\[0 x i16\]\*|ptr}}, [[USIZE]] } @return_slice({{\[0 x i16\]\*|ptr}} noalias noundef nonnull readonly align 2 %x.0, [[USIZE]] noundef %x.1)
// CHERI: { {{\[0 x i16\] addrspace\(200\)\*|ptr addrspace\(200\)}}, [[USIZE]] } @return_slice({{\[0 x i16\] addrspace\(200\)\*|ptr addrspace\(200\)}} noalias noundef nonnull readonly align 2 %x.0, [[USIZE]] noundef %x.1)
#[no_mangle]
pub fn return_slice(x: &[u16]) -> &[u16] {
  x
}

// CHECK: { i16, i16 } @enum_id_1(i16 noundef %x.0, i16 %x.1)
#[no_mangle]
pub fn enum_id_1(x: Option<Result<u16, u16>>) -> Option<Result<u16, u16>> {
  x
}

// CHECK: { i8, i8 } @enum_id_2(i1 noundef zeroext %x.0, i8 %x.1)
#[no_mangle]
pub fn enum_id_2(x: Option<u8>) -> Option<u8> {
  x
}

// NONCHERI: { {{\{\}\*|ptr}}, {{.+}} } @dyn_star({{\{\}\*|ptr}} noundef %x.0, {{.+}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %x.1)
// CHERI: { {{\{\} addrspace\(200\)\*|ptr addrspace\(200\)}}, {{.+}} } @dyn_star({{\{\} addrspace\(200\)\*|ptr addrspace\(200\)}} noundef %x.0, {{.+}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %x.1)
// Expect an ABI something like `{ {}*, [3 x i64]* }`, but that's hard to match on generically,
// so do like the `trait_box` test and just match on `{{.+}}` for the vtable.
#[no_mangle]
pub fn dyn_star(x: dyn* Drop) -> dyn* Drop {
  x
}
