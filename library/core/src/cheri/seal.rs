//! Sealing pointers.

/// A sealed pointer.
#[lang = "cheri_sealed_capability"]
#[repr(transparent)]
pub struct SealedCapability<T: crate::marker::PointerLike>(T);

impl<T: core::marker::PointerLike> SealedCapability<T> {
    /// Unseal the capability.
    pub unsafe fn unseal<K: core::marker::PointerLike>(self, key: K) -> T {
        unsafe {
            crate::intrinsics::cheri::cheri_unseal(self, key)
        }
    }
}

impl<T: core::marker::PointerLike + 'static> crate::fmt::Debug for SealedCapability<T> {
    fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result {
        write!(f, "*sealed {:?}", crate::any::TypeId::of::<T>())
    }
}
