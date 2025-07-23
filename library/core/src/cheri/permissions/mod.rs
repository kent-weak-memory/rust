//! CHERI permissions.

mod permissions_impl;
pub use permissions_impl::*;

/// Encapsulates a set of permissions.
#[derive(Debug)]
pub struct PermissionSet {
    raw_permissions: usize,
}

impl PermissionSet {
    /// Create a new permissions set from its raw representation.
    pub const fn from_raw(raw_permissions: usize) -> Self {
        Self { raw_permissions }
    }

    /// Create a new empty permissions set.
    pub const fn empty() -> Self {
        Self { raw_permissions: 0 }
    }

    /// Get the representation of this set as a [`usize`].
    pub const fn as_raw(&self) -> usize {
        self.raw_permissions
    }

    /// Create a new set of permissions from an iterator of [`Permission`].
    pub fn from_iter<I: Iterator<Item = Permission>>(iter: I) -> Self {
        let mut ret = Self::empty();
        let iter = iter.into_iter();

        for i in iter {
            ret.add_permission(i);
        }

        ret
    }

    /// Check if this set contains the given `permission`.
    pub const fn contains(&self, permission: Permission) -> bool {
        permission.bit() & self.raw_permissions == permission.bit()
    }

    /// Add a permission to this set.
    pub const fn add_permission(&mut self, permission: Permission) {
        self.raw_permissions |= permission.bit();
    }
}

impl core::fmt::Display for PermissionSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;
        if self.contains(Permission::Global) {
            write!(f, "G")?;
        }

        if self.contains(Permission::LoadGlobal) {
            write!(f, "<Lg>")?;
        }

        if self.contains(Permission::Store) {
            write!(f, "W")?;
        }

        if self.contains(Permission::LoadMutable) {
            write!(f, "m")?;
        }

        if self.contains(Permission::StoreLocal) {
            write!(f, "<Sl>")?;
        }

        if self.contains(Permission::Load) {
            write!(f, "R")?;
        }

        if self.contains(Permission::LoadStoreCapability) {
            write!(f, "c")?;
        }

        if self.contains(Permission::AccessSystemRegisters) {
            write!(f, "s")?;
        }

        if self.contains(Permission::Execute) {
            write!(f, "X")?;
        }

        if self.contains(Permission::Unseal) {
            write!(f, "u")?;
        }

        if self.contains(Permission::Seal) {
            write!(f, "S")?;
        }

        write!(f, ")")
    }
}

#[macro_export]
/// A macro to create a [`PermissionSet`] from a list of [`Permission`], evaluated at compile-time.
macro_rules! perms {
    {$($p: expr),*} => {
        const {
            let mut ret = PermissionSet::empty();
            $(
                ret.add_permission($p);
            )*
            ret
        }
    };
}

pub use perms;
