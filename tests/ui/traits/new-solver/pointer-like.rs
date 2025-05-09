// compile-flags: -Ztrait-solver=next

#![feature(cfg_target_abi)]
#![feature(pointer_like_trait)]

use std::marker::PointerLike;

fn require_(_: impl PointerLike) {}

fn main() {
    #[cfg(not(target_abi = "purecap"))]
    require_(1usize);
    require_(1u16);
    //~^ ERROR `u16` needs to have the same ABI as a pointer
    require_(&1i16);
}
