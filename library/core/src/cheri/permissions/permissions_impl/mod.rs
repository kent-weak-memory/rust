#[cfg(not(target_family = "cheriot"))]
mod cheri;
#[cfg(not(target_family = "cheriot"))]
pub use cheri::*;

#[cfg(target_family = "cheriot")]
mod cheriot;
#[cfg(target_family = "cheriot")]
pub use cheriot::*;
