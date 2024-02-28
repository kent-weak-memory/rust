// revisions: lib staticlib
// ignore-emscripten default visibility is hidden
// compile-flags: -O
// `#[no_mangle]`d static variables always have external linkage, i.e., no `internal` in their
// definitions

#![cfg_attr(lib, crate_type = "lib")]
#![cfg_attr(staticlib, crate_type = "staticlib")]

// NONCHERI: @A = local_unnamed_addr constant
// CHERI: @A = local_unnamed_addr addrspace(200) constant
#[no_mangle]
static A: u8 = 0;

// NONCHERI: @B = local_unnamed_addr global
// CHERI: @B = local_unnamed_addr addrspace(200) global
#[no_mangle]
static mut B: u8 = 0;

// NONCHERI: @C = local_unnamed_addr constant
// CHERI: @C = local_unnamed_addr addrspace(200) constant
#[no_mangle]
pub static C: u8 = 0;

// NONCHERI: @D = local_unnamed_addr global
// CHERI: @D = local_unnamed_addr addrspace(200) global
#[no_mangle]
pub static mut D: u8 = 0;

mod private {
    // NONCHERI: @E = local_unnamed_addr constant
    // CHERI: @E = local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    static E: u8 = 0;

    // NONCHERI: @F = local_unnamed_addr global
    // CHERI: @F = local_unnamed_addr addrspace(200) global
    #[no_mangle]
    static mut F: u8 = 0;

    // NONCHERI: @G = local_unnamed_addr constant
    // CHERI: @G = local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    pub static G: u8 = 0;

    // NONCHERI: @H = local_unnamed_addr global
    // CHERI: @H = local_unnamed_addr addrspace(200) global
    #[no_mangle]
    pub static mut H: u8 = 0;
}

const HIDDEN: () = {
    // NONCHERI: @I = local_unnamed_addr constant
    // CHERI: @I = local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    static I: u8 = 0;

    // NONCHERI: @J = local_unnamed_addr global
    // CHERI: @J = local_unnamed_addr addrspace(200) global
    #[no_mangle]
    static mut J: u8 = 0;

    // NONCHERI: @K = local_unnamed_addr constant
    // CHERI: @K = local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    pub static K: u8 = 0;

    // NONCHERI: @L = local_unnamed_addr global
    // CHERI: @L = local_unnamed_addr addrspace(200) global
    #[no_mangle]
    pub static mut L: u8 = 0;
};

fn x() {
    // NONCHERI: @M = local_unnamed_addr constant
    // CHERI: @M = local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    static M: fn() = x;

    // NONCHERI: @N = local_unnamed_addr global
    // CHERI: @N = local_unnamed_addr addrspace(200) global
    #[no_mangle]
    static mut N: u8 = 0;

    // NONCHERI: @O = local_unnamed_addr constant
    // CHERI: @O = local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    pub static O: u8 = 0;

    // NONCHERI: @P = local_unnamed_addr global
    // CHERI: @P = local_unnamed_addr addrspace(200) global
    #[no_mangle]
    pub static mut P: u8 = 0;
}
