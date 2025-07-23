//! CHERIoT permissions.

/// The complete set of architectural permissions.
// Taken from cheriot-rtos' `cheri.h`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Permission {
    /// Capability refers to global memory (this capability may be stored anywhere).
    Global = 0,

    /// Global capabilities can be loaded through this capability.  Without this permission, any
    /// capability loaded via this capability will have `Global` and `LoadGlobal` removed.
    LoadGlobal = 1,

    /// Capability may be used to store.  Any store via a capability without this permission will
    /// trap.
    Store = 2,

    /// Capabilities with store permission may be loaded through this capability.  Without this,
    /// any loaded capability will have `LoadMutable` and `Store` removed.
    LoadMutable = 3,

    /// This capability may be used to store capabilities that do not have `Global` permission.
    StoreLocal = 4,

    /// This capability can be used to load.
    Load = 5,

    /// Any load and store permissions on this capability convey the right to load or store
    /// capabilities in addition to data.
    LoadStoreCapability = 6,

    /// If installed as the program counter capability, running code may access privileged system
    /// registers.
    AccessSystemRegisters = 7,

    /// This capability may be used as a jump target and used to execute instructions.
    Execute = 8,

    /// This capability may be used to unseal other capabilities.  The 'address' range is in the
    /// sealing type namespace and not in the memory namespace.
    Unseal = 9,

    /// This capability may be used to seal other capabilities.  The 'address' range is in the
    /// sealing type namespace and not in the memory namespace.
    Seal = 10,

    /// Software defined permission bit, no architectural meaning.
    User0 = 11,
}

impl Permission {
    /// Get the representation of this permission as an `usize`.
    pub const fn bit(&self) -> usize {
        1 << (*self as usize)
    }
}
