// check-pass
// ignore-purecap: `usize` is not compatible with this use on CHERI targets

#![feature(dyn_star)]
//~^ WARN the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes

fn _foo() -> dyn* Unpin {
    4usize
}

fn main() {}
