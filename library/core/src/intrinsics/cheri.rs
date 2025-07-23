//! Some intrinsics specific to CHERI systems.

/// Create a new *mut <T> pointer for any <T>.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
#[rustc_allow_const_fn_unstable(core_intrinsics)]
pub const fn cheri_null_mut<T: ?Sized + crate::ptr::Thin>() -> *mut T;

/// Retrieve the address of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_address_get<T: core::marker::PointerLike>(ptr: T) -> usize;

/// Set the address of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_address_set<T: core::marker::PointerLike>(ptr: T, addr: usize) -> T;

/// Increment the offset of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_offset_increment<T: core::marker::PointerLike>(ptr: T, offset: usize) -> T;

/// Retrieve the base of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_base_get<T: core::marker::PointerLike>(ptr: T) -> usize;

/// Retrieve the length of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_length_get<T: core::marker::PointerLike>(ptr: T) -> usize;

/// Retrieve the top of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_top_get<T: core::marker::PointerLike>(ptr: T) -> usize;

/// Clear the tag of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_tag_clear<T: core::marker::PointerLike>(ptr: T);

/// Get the tag of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_tag_get<T: core::marker::PointerLike>(ptr: T) -> bool;

/// Compare two capabilities for exact equality.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_is_equal_exact<T: core::marker::PointerLike>(ptr1: T, ptr2: T) -> bool;

/// Get the raw permissions of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_permissions_get<T: core::marker::PointerLike>(ptr: T) -> usize;

/// Augment the permissions of the capability (computing the logical and).
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_permissions_and<T: core::marker::PointerLike>(ptr: T, perms: usize) -> T;

/// Get the type of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_type_get<T: core::marker::PointerLike>(ptr: T) -> u32;

/// Seal the capability with the given key.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_seal<T: core::marker::PointerLike, K: core::marker::PointerLike>(
    ptr: T,
    key: K,
) -> crate::cheri::seal::SealedCapability<T>;

/// Unseal the capability with the given key.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_unseal<T: core::marker::PointerLike, K: core::marker::PointerLike>(
    ptr: crate::cheri::seal::SealedCapability<T>,
    key: K,
) -> T;

/// Set the bounds of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_bounds_set<T: core::marker::PointerLike>(ptr: T, bounds: usize) -> T;

/// Set the bounds of the capability without any rounding.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_bounds_set_exact<T: core::marker::PointerLike>(ptr: T, bounds: usize) -> T;

/// Test if `ptr1` is a subset of `ptr2`.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_subset_test<T: core::marker::PointerLike>(ptr1: T, ptr2: T) -> bool;

/// Get the representable alignment mask for the given length.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_representable_alignment_mask(len: usize) -> usize;

/// Get the rounded representable length for the given length.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_round_representable_length(len: usize) -> usize;
