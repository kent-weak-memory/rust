//! [CHERI] - Capability Hardware Enhanced RISC Instructions - is a joint research project to
//! revisit fundamental design choices in hardware and software to dramatically improve system
//! security.
//!
//! CHERI extends conventional hardware Instruction-Set Architectures (ISAs) with new architectural
//! features to enable fine-grained memory protection and highly scalable software
//! compartmentalization.
//!
//! As the name suggests, CHERI platforms use [capabilities] in lieu of integral pointers.
//! Capabilities, in a nutshell, are memory addresses augmented with metadata that describes
//! important informations with respect to the capability itself: an example could be a pair `(addr,
//! metadata)` where `addr` is a memory address, and `metadata` describes the length of the memory
//! region this capability can access, whether it can write it or read it and so forth.
//!
//! This module describes what a capability is for CHERI platforms.
//!
//! [CHERI]: https://www.cl.cam.ac.uk/research/security/ctsrd/cheri/
//! [capabilities]: https://en.wikipedia.org/wiki/Capability-based_addressing
// TODO(xdoardo): more docs ^

use seal::SealedCapability;

pub mod permissions;
use permissions::PermissionSet;

pub mod seal;

/// A capability is a concept specific to CHERI systems.
/// In summary, it is a well-defined non-integral pointer.
pub trait Capability<'a>: core::marker::PointerLike + Sized {
    /// Check the validity of this capability.
    fn is_valid(&'a self) -> bool;

    #[inline]
    /// Check the invalidity of this capability.
    fn is_invalid(&'a self) -> bool {
        !self.is_valid()
    }

    /// Get the address of this capability.
    fn addr(&'a self) -> usize;

    /// Creates a new pointer with the given address and the metadata of `self`.
    unsafe fn with_addr(self, addr: usize) -> Self;

    /// Creates a new pointer with the address of self plus `offset` and the metadata of `self`.
    unsafe fn with_offset(self, offset: usize) -> Self;

    /// Get the base of this capability.
    fn base(&'a self) -> usize;

    /// Get the length of this capability.
    fn length(&'a self) -> usize;

    /// Get the top of this capability.
    fn top(&'a self) -> usize;

    /// Clear the tag of this capability.
    unsafe fn set_invalid(&'a mut self);

    /// Get the tag of this capability.
    fn is_equal(&'a self, other: &Self) -> bool;

    /// Get the tag of this capability.
    fn is_subset(&'a self, other: &Self) -> bool;

    /// Get the permissions of this capability.
    fn permissions(&'a self) -> PermissionSet;

    /// Create a new capability, keeping _at most_ the permissions already given to `self`. In
    /// practice, this means that the resulting permissions for the resulting capability will be the
    /// logical conjunction of `perms` and the `self.permissions()`.
    fn with_permissions(self, perms: PermissionSet) -> Self;

    /// Seal this capability.
    fn seal<K: core::marker::PointerLike>(self, key: K) -> SealedCapability<Self>;

    /// Set the bounds of this capability.
    ///
    /// Note: might be rounded.
    unsafe fn with_bounds(self, bounds: usize) -> Self;

    /// Set the bounds of this capability.
    unsafe fn with_bounds_exact(self, bounds: usize) -> Self;
}

impl<'a, T: core::marker::PointerLike + Copy> Capability<'a> for T {
    #[inline(always)]
    /// Get the address of this capability.
    fn addr(&'a self) -> usize {
        crate::intrinsics::cheri::cheri_address_get(*self)
    }

    #[inline(always)]
    unsafe fn with_addr(self, addr: usize) -> Self {
        unsafe { crate::intrinsics::cheri::cheri_address_set(self, addr) }
    }

    #[inline(always)]
    /// Get the base of this capability.
    fn base(&'a self) -> usize {
        crate::intrinsics::cheri::cheri_base_get(*self)
    }

    #[inline(always)]
    /// Get the top of this capability.
    fn top(&'a self) -> usize {
        crate::intrinsics::cheri::cheri_top_get(*self)
    }

    #[inline(always)]
    /// Get the tag of this capability.
    fn is_valid(&'a self) -> bool {
        crate::intrinsics::cheri::cheri_tag_get(*self)
    }

    #[inline(always)]
    unsafe fn with_offset(self, offset: usize) -> Self {
        unsafe { crate::intrinsics::cheri::cheri_offset_increment(self, offset) }
    }

    #[inline(always)]
    fn length(&'a self) -> usize {
        crate::intrinsics::cheri::cheri_length_get(*self)
    }

    #[inline(always)]
    unsafe fn set_invalid(&'a mut self) {
        unsafe { crate::intrinsics::cheri::cheri_tag_clear(*self) }
    }

    #[inline(always)]
    fn is_equal(&'a self, other: &Self) -> bool {
        crate::intrinsics::cheri::cheri_is_equal_exact(*self, *other)
    }

    #[inline(always)]
    fn is_subset(&'a self, other: &Self) -> bool {
        crate::intrinsics::cheri::cheri_subset_test(*self, *other)
    }

    fn permissions(&'a self) -> PermissionSet {
        PermissionSet::from_raw(crate::intrinsics::cheri::cheri_permissions_get(*self))
    }

    fn with_permissions(self, perms: PermissionSet) -> Self {
        unsafe { crate::intrinsics::cheri::cheri_permissions_and(self, perms.as_raw()) }
    }

    #[inline(always)]
    fn seal<K: crate::marker::PointerLike>(self, key: K) -> SealedCapability<Self> {
        unsafe { crate::intrinsics::cheri::cheri_seal(self, key) }
    }

    unsafe fn with_bounds(self, bounds: usize) -> Self {
        unsafe { crate::intrinsics::cheri::cheri_bounds_set(self, bounds) }
    }

    unsafe fn with_bounds_exact(self, bounds: usize) -> Self {
        unsafe { crate::intrinsics::cheri::cheri_bounds_set_exact(self, bounds) }
    }
}

impl<'a, T> Capability<'a> for &'a mut T {
    #[inline(always)]
    fn addr(&'a self) -> usize {
        crate::intrinsics::cheri::cheri_address_get(&**self)
    }

    #[inline(always)]
    unsafe fn with_addr(self, addr: usize) -> Self {
        unsafe { crate::intrinsics::cheri::cheri_address_set(self, addr) }
    }

    #[inline(always)]
    fn base(&'a self) -> usize {
        crate::intrinsics::cheri::cheri_base_get(&**self)
    }

    #[inline(always)]
    fn top(&'a self) -> usize {
        crate::intrinsics::cheri::cheri_top_get(&**self)
    }

    #[inline(always)]
    fn is_valid(&'a self) -> bool {
        crate::intrinsics::cheri::cheri_tag_get(&**self)
    }

    #[inline(always)]
    unsafe fn with_offset(self, offset: usize) -> Self {
        unsafe { crate::intrinsics::cheri::cheri_offset_increment(self, offset) }
    }

    #[inline(always)]
    fn length(&'a self) -> usize {
        crate::intrinsics::cheri::cheri_length_get(&**self)
    }

    #[inline(always)]
    unsafe fn set_invalid(&'a mut self) {
        unsafe { crate::intrinsics::cheri::cheri_tag_clear(&mut **self) }
    }

    #[inline(always)]
    fn is_equal(&'a self, other: &Self) -> bool {
        crate::intrinsics::cheri::cheri_is_equal_exact(&**self, *other)
    }

    #[inline(always)]
    fn is_subset(&'a self, other: &Self) -> bool {
        crate::intrinsics::cheri::cheri_subset_test(&**self, *other)
    }

    fn permissions(&'a self) -> PermissionSet {
        PermissionSet::from_raw(crate::intrinsics::cheri::cheri_permissions_get(&**self))
    }

    fn with_permissions(self, perms: PermissionSet) -> Self {
        unsafe { crate::intrinsics::cheri::cheri_permissions_and(self, perms.as_raw()) }
    }

    #[inline(always)]
    fn seal<K: crate::marker::PointerLike>(self, key: K) -> SealedCapability<Self> {
        unsafe { crate::intrinsics::cheri::cheri_seal(self, key) }
    }

    unsafe fn with_bounds(self, bounds: usize) -> Self {
        unsafe { crate::intrinsics::cheri::cheri_bounds_set(self, bounds) }
    }

    unsafe fn with_bounds_exact(self, bounds: usize) -> Self {
        unsafe { crate::intrinsics::cheri::cheri_bounds_set_exact(self, bounds) }
    }
}
