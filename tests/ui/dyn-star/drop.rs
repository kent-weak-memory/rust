// run-pass
// check-run-results
// ignore-purecap: `usize` is not compatible with this use on CHERI targets
#![feature(dyn_star)]
#![allow(incomplete_features)]

use std::fmt::Debug;

#[derive(Debug)]
struct Foo(usize);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("destructor called");
    }
}

fn make_dyn_star(i: Foo) {
    let _dyn_i: dyn* Debug = i;
}

fn main() {
    make_dyn_star(Foo(42));
}
