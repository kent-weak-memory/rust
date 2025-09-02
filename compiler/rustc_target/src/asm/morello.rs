//! Inline assembly for Morello (AArch64 with CHERI).
//! This is just a copy of the `aarch64.rs` version with an extra capability
//! register class.

use super::{InlineAsmArch, InlineAsmType};
use crate::spec::{RelocModel, Target};
use rustc_data_structures::fx::FxIndexSet;
use rustc_macros::HashStable_Generic;
use rustc_span::Symbol;
use std::fmt;

def_reg_class! {
    Morello MorelloInlineAsmRegClass {
        reg,
        vreg,
        vreg_low16,
        preg,
    }
}

impl MorelloInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        match self {
            Self::reg => &['w', 'x', 'C'],
            Self::vreg | Self::vreg_low16 => &['b', 'h', 's', 'd', 'q', 'v'],
            Self::preg => &[],
        }
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        ty: InlineAsmType,
    ) -> Option<(char, &'static str)> {
        match self {
            Self::reg => match ty.size().bits() {
                128 => Some(('C', "c0")),
                64 => None,
                _ => Some(('w', "w0")),
            },
            Self::vreg | Self::vreg_low16 => match ty.size().bits() {
                8 => Some(('b', "b0")),
                16 => Some(('h', "h0")),
                32 => Some(('s', "s0")),
                64 => Some(('d', "d0")),
                128 => Some(('q', "q0")),
                _ => None,
            },
            Self::preg => None,
        }
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<(char, &'static str)> {
        match self {
            Self::reg => Some(('x', "x0")),
            Self::vreg | Self::vreg_low16 => Some(('v', "v0")),
            Self::preg => None,
        }
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match self {
            // TODO(seharris): does putting 128 bit integers in capability
            //                 registers make anything peculiar happen?
            Self::reg => types! { _: I8, I16, I32, I64, I128, F32, F64; },
            Self::vreg | Self::vreg_low16 => types! {
                neon: I8, I16, I32, I64, F32, F64,
                    VecI8(8), VecI16(4), VecI32(2), VecI64(1), VecF32(2), VecF64(1),
                    VecI8(16), VecI16(8), VecI32(4), VecI64(2), VecF32(4), VecF64(2);
            },
            Self::preg => &[],
        }
    }
}

pub fn target_reserves_x18(target: &Target) -> bool {
    target.os == "android" || target.os == "fuchsia" || target.is_like_osx || target.is_like_windows
}

fn reserved_x18(
    _arch: InlineAsmArch,
    _reloc_model: RelocModel,
    _target_features: &FxIndexSet<Symbol>,
    target: &Target,
    _is_clobber: bool,
) -> Result<(), &'static str> {
    if target_reserves_x18(target) {
        Err("c18 is a reserved register on this target")
    } else {
        Ok(())
    }
}

def_regs! {
    Morello MorelloInlineAsmReg MorelloInlineAsmRegClass {
        c0: reg = ["c0", "x0", "w0"],
        c1: reg = ["c1", "x1", "w1"],
        c2: reg = ["c2", "x2", "w2"],
        c3: reg = ["c3", "x3", "w3"],
        c4: reg = ["c4", "x4", "w4"],
        c5: reg = ["c5", "x5", "w5"],
        c6: reg = ["c6", "x6", "w6"],
        c7: reg = ["c7", "x7", "w7"],
        c8: reg = ["c8", "x8", "w8"],
        c9: reg = ["c9", "x9", "w9"],
        c10: reg = ["c10", "x10", "w10"],
        c11: reg = ["c11", "x11", "w11"],
        c12: reg = ["c12", "x12", "w12"],
        c13: reg = ["c13", "x13", "w13"],
        c14: reg = ["c14", "x14", "w14"],
        c15: reg = ["c15", "x15", "w15"],
        c16: reg = ["c16", "x16", "w16"],
        c17: reg = ["c17", "x17", "w17"],
        c18: reg = ["c18", "x18", "w18"] % reserved_x18,
        c20: reg = ["c20", "x20", "w20"],
        c21: reg = ["c21", "x21", "w21"],
        c22: reg = ["c22", "x22", "w22"],
        c23: reg = ["c23", "x23", "w23"],
        c24: reg = ["c24", "x24", "w24"],
        c25: reg = ["c25", "x25", "w25"],
        c26: reg = ["c26", "x26", "w26"],
        c27: reg = ["c27", "x27", "w27"],
        c28: reg = ["c28", "x28", "w28"],
        c30: reg = ["c30", "x30", "w30", "lr", "wlr"],
        v0: vreg, vreg_low16 = ["v0", "b0", "h0", "s0", "d0", "q0", "z0"],
        v1: vreg, vreg_low16 = ["v1", "b1", "h1", "s1", "d1", "q1", "z1"],
        v2: vreg, vreg_low16 = ["v2", "b2", "h2", "s2", "d2", "q2", "z2"],
        v3: vreg, vreg_low16 = ["v3", "b3", "h3", "s3", "d3", "q3", "z3"],
        v4: vreg, vreg_low16 = ["v4", "b4", "h4", "s4", "d4", "q4", "z4"],
        v5: vreg, vreg_low16 = ["v5", "b5", "h5", "s5", "d5", "q5", "z5"],
        v6: vreg, vreg_low16 = ["v6", "b6", "h6", "s6", "d6", "q6", "z6"],
        v7: vreg, vreg_low16 = ["v7", "b7", "h7", "s7", "d7", "q7", "z7"],
        v8: vreg, vreg_low16 = ["v8", "b8", "h8", "s8", "d8", "q8", "z8"],
        v9: vreg, vreg_low16 = ["v9", "b9", "h9", "s9", "d9", "q9", "z9"],
        v10: vreg, vreg_low16 = ["v10", "b10", "h10", "s10", "d10", "q10", "z10"],
        v11: vreg, vreg_low16 = ["v11", "b11", "h11", "s11", "d11", "q11", "z11"],
        v12: vreg, vreg_low16 = ["v12", "b12", "h12", "s12", "d12", "q12", "z12"],
        v13: vreg, vreg_low16 = ["v13", "b13", "h13", "s13", "d13", "q13", "z13"],
        v14: vreg, vreg_low16 = ["v14", "b14", "h14", "s14", "d14", "q14", "z14"],
        v15: vreg, vreg_low16 = ["v15", "b15", "h15", "s15", "d15", "q15", "z15"],
        v16: vreg = ["v16", "b16", "h16", "s16", "d16", "q16", "z16"],
        v17: vreg = ["v17", "b17", "h17", "s17", "d17", "q17", "z17"],
        v18: vreg = ["v18", "b18", "h18", "s18", "d18", "q18", "z18"],
        v19: vreg = ["v19", "b19", "h19", "s19", "d19", "q19", "z19"],
        v20: vreg = ["v20", "b20", "h20", "s20", "d20", "q20", "z20"],
        v21: vreg = ["v21", "b21", "h21", "s21", "d21", "q21", "z21"],
        v22: vreg = ["v22", "b22", "h22", "s22", "d22", "q22", "z22"],
        v23: vreg = ["v23", "b23", "h23", "s23", "d23", "q23", "z23"],
        v24: vreg = ["v24", "b24", "h24", "s24", "d24", "q24", "z24"],
        v25: vreg = ["v25", "b25", "h25", "s25", "d25", "q25", "z25"],
        v26: vreg = ["v26", "b26", "h26", "s26", "d26", "q26", "z26"],
        v27: vreg = ["v27", "b27", "h27", "s27", "d27", "q27", "z27"],
        v28: vreg = ["v28", "b28", "h28", "s28", "d28", "q28", "z28"],
        v29: vreg = ["v29", "b29", "h29", "s29", "d29", "q29", "z29"],
        v30: vreg = ["v30", "b30", "h30", "s30", "d30", "q30", "z30"],
        v31: vreg = ["v31", "b31", "h31", "s31", "d31", "q31", "z31"],
        p0: preg = ["p0"],
        p1: preg = ["p1"],
        p2: preg = ["p2"],
        p3: preg = ["p3"],
        p4: preg = ["p4"],
        p5: preg = ["p5"],
        p6: preg = ["p6"],
        p7: preg = ["p7"],
        p8: preg = ["p8"],
        p9: preg = ["p9"],
        p10: preg = ["p10"],
        p11: preg = ["p11"],
        p12: preg = ["p12"],
        p13: preg = ["p13"],
        p14: preg = ["p14"],
        p15: preg = ["p15"],
        ffr: preg = ["ffr"],
        #error = ["c19", "x19", "w19"] =>
            "c19 is used internally by LLVM and cannot be used as an operand for inline asm",
        #error = ["c29", "x29", "w29", "fp", "wfp"] =>
            "the frame pointer cannot be used as an operand for inline asm",
        #error = ["csp", "sp", "wsp"] =>
            "the stack pointer cannot be used as an operand for inline asm",
        #error = ["czr", "xzr", "wzr"] =>
            "the zero register cannot be used as an operand for inline asm",
    }
}

impl MorelloInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        modifier: Option<char>,
    ) -> fmt::Result {
        let (prefix, index) = if (self as u32) < Self::v0 as u32 {
            (modifier.unwrap_or('x'), self as u32 - Self::c0 as u32)
        } else {
            (modifier.unwrap_or('v'), self as u32 - Self::v0 as u32)
        };
        assert!(index < 32);
        write!(out, "{prefix}{index}")
    }
}
