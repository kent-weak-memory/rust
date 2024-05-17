// revisions: lib staticlib
// ignore-emscripten default visibility is hidden
// compile-flags: -O
// [lib] compile-flags: --crate-type lib
// [staticlib] compile-flags: --crate-type staticlib
// `#[no_mangle]`d static variables always have external linkage, i.e., no `internal` in their
// definitions

// NONCHERI: @A = {{(dso_local )?}}local_unnamed_addr constant
// CHERI: @A = {{(dso_local )?}}local_unnamed_addr addrspace(200) constant
#[no_mangle]
static A: u8 = 0;

// NONCHERI: @B = {{(dso_local )?}}local_unnamed_addr global
// CHERI: @B = {{(dso_local )?}}local_unnamed_addr addrspace(200) global
#[no_mangle]
static mut B: u8 = 0;

// NONCHERI: @C = {{(dso_local )?}}local_unnamed_addr constant
// CHERI: @C = {{(dso_local )?}}local_unnamed_addr addrspace(200) constant
#[no_mangle]
pub static C: u8 = 0;

// NONCHERI: @D = {{(dso_local )?}}local_unnamed_addr global
// CHERI: @D = {{(dso_local )?}}local_unnamed_addr addrspace(200) global
#[no_mangle]
pub static mut D: u8 = 0;

mod private {
    // NONCHERI: @E = {{(dso_local )?}}local_unnamed_addr constant
    // CHERI: @E = {{(dso_local )?}}local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    static E: u8 = 0;

    // NONCHERI: @F = {{(dso_local )?}}local_unnamed_addr global
    // CHERI: @F = {{(dso_local )?}}local_unnamed_addr addrspace(200) global
    #[no_mangle]
    static mut F: u8 = 0;

    // NONCHERI: @G = {{(dso_local )?}}local_unnamed_addr constant
    // CHERI: @G = {{(dso_local )?}}local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    pub static G: u8 = 0;

    // NONCHERI: @H = {{(dso_local )?}}local_unnamed_addr global
    // CHERI: @H = {{(dso_local )?}}local_unnamed_addr addrspace(200) global
    #[no_mangle]
    pub static mut H: u8 = 0;
}

const HIDDEN: () = {
    // NONCHERI: @I = {{(dso_local )?}}local_unnamed_addr constant
    // CHERI: @I = {{(dso_local )?}}local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    static I: u8 = 0;

    // NONCHERI: @J = {{(dso_local )?}}local_unnamed_addr global
    // CHERI: @J = {{(dso_local )?}}local_unnamed_addr addrspace(200) global
    #[no_mangle]
    static mut J: u8 = 0;

    // NONCHERI: @K = {{(dso_local )?}}local_unnamed_addr constant
    // CHERI: @K = {{(dso_local )?}}local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    pub static K: u8 = 0;

    // NONCHERI: @L = {{(dso_local )?}}local_unnamed_addr global
    // CHERI: @L = {{(dso_local )?}}local_unnamed_addr addrspace(200) global
    #[no_mangle]
    pub static mut L: u8 = 0;
};

fn x() {
    // NONCHERI: @M = {{(dso_local )?}}local_unnamed_addr constant
    // CHERI: @M = {{(dso_local )?}}local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    static M: fn() = x;

    // NONCHERI: @N = {{(dso_local )?}}local_unnamed_addr global
    // CHERI: @N = {{(dso_local )?}}local_unnamed_addr addrspace(200) global
    #[no_mangle]
    static mut N: u8 = 0;

    // NONCHERI: @O = {{(dso_local )?}}local_unnamed_addr constant
    // CHERI: @O = {{(dso_local )?}}local_unnamed_addr addrspace(200) constant
    #[no_mangle]
    pub static O: u8 = 0;

    // NONCHERI: @P = {{(dso_local )?}}local_unnamed_addr global
    // CHERI: @P = {{(dso_local )?}}local_unnamed_addr addrspace(200) global
    #[no_mangle]
    pub static mut P: u8 = 0;
}
