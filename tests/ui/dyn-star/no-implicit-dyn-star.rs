// aux-build:dyn-star-foreign.rs
// ignore-purecap: `usize` is not compatible with this use on CHERI targets

extern crate dyn_star_foreign;

fn main() {
    dyn_star_foreign::require_dyn_star_display(1usize);
    //~^ ERROR mismatched types
}
