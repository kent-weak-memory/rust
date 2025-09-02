// only-aarch64-unknown-freebsd-purecap
// compile-flags: -C target-feature=+neon

// Check register notation on Morello (AArch64 CHERI).

#![feature(asm_const)]

use std::arch::asm;

fn main() {
    let mut foo = 0;
    let mut bar = 0;
    let pointer = std::ptr::null::<u32>();
    unsafe {
        // Valid registers.

        asm!("{:w}", in(reg) foo);
        asm!("{:x}", in(reg) foo);
        // asm!("{:C}", in(reg) foo); TODO(seharris): re-enable when LLVM supports `{:C}`.

        // Bad register/register class

        asm!("{}", in(foo) foo);
        //~^ ERROR invalid register class `foo`: unknown register class
        asm!("", in("foo") foo);
        //~^ ERROR invalid register `foo`: unknown register
        asm!("{:z}", in(reg) foo);
        //~^ ERROR invalid asm template modifier for this register class
        asm!("{:r}", in(vreg) foo);
        //~^ ERROR invalid asm template modifier for this register class
        asm!("{:r}", in(vreg_low16) foo);
        //~^ ERROR invalid asm template modifier for this register class
        asm!("{:a}", const 0);
        //~^ ERROR asm template modifiers are not allowed for `const` arguments
        asm!("{:a}", sym main);
        //~^ ERROR asm template modifiers are not allowed for `sym` arguments
        asm!("", in("x29") foo);
        //~^ ERROR invalid register `x29`: the frame pointer cannot be used as an operand
        asm!("", in("sp") foo);
        //~^ ERROR invalid register `sp`: the stack pointer cannot be used as an operand
        asm!("", in("xzr") foo);
        //~^ ERROR invalid register `xzr`: the zero register cannot be used as an operand
        asm!("", in("x19") foo);
        //~^ ERROR invalid register `x19`: c19 is used internally by LLVM and cannot be used as an operand for inline asm

        asm!("", in("p0") foo);
        //~^ ERROR register class `preg` can only be used as a clobber, not as an input or output
        //~| ERROR type `i32` cannot be used with this register class
        asm!("", out("p0") _);
        asm!("{}", in(preg) foo);
        //~^ ERROR register class `preg` can only be used as a clobber, not as an input or output
        //~| ERROR type `i32` cannot be used with this register class
        asm!("{}", out(preg) _);
        //~^ ERROR register class `preg` can only be used as a clobber, not as an input or output

        asm!("", in("c29") foo);
        //~^ ERROR invalid register `c29`: the frame pointer cannot be used as an operand
        asm!("", in("csp") foo);
        //~^ ERROR invalid register `csp`: the stack pointer cannot be used as an operand
        asm!("", in("czr") foo);
        //~^ ERROR invalid register `czr`: the zero register cannot be used as an operand
        asm!("", in("c19") foo);
        //~^ ERROR invalid register `c19`: c19 is used internally by LLVM and cannot be used as an operand for inline asm

        // Explicit register conflicts
        // (except in/lateout which don't conflict)

        asm!("", in("x0") foo, in("w0") bar);
        //~^ ERROR register `c0` conflicts with register `c0`
        asm!("", in("x0") foo, out("x0") bar);
        //~^ ERROR register `c0` conflicts with register `c0`
        asm!("", in("w0") foo, lateout("w0") bar);
        asm!("", in("v0") foo, in("q0") bar);
        //~^ ERROR register `v0` conflicts with register `v0`
        asm!("", in("v0") foo, out("q0") bar);
        //~^ ERROR register `v0` conflicts with register `v0`
        asm!("", in("v0") foo, lateout("q0") bar);

        asm!("", in("c0") foo, in("w0") bar);
        //~^ ERROR register `c0` conflicts with register `c0`
        asm!("", in("c0") foo, in("x0") bar);
        //~^ ERROR register `c0` conflicts with register `c0`
        asm!("", in("c0") foo, out("c0") bar);
        //~^ ERROR register `c0` conflicts with register `c0`
        asm!("", in("c0") foo, lateout("c0") bar);
    }
}
