use crate::spec::{LinkerFlavor, Target, TargetOptions};

// This target if is for the Android v7a ABI in thumb mode with
// NEON unconditionally enabled and, therefore, with 32 FPU registers
// enabled as well. See section A2.6.2 on page A2-56 in
// https://static.docs.arm.com/ddi0406/cd/DDI0406C_d_armv7ar_arm.pdf

// See https://developer.android.com/ndk/guides/abis.html#v7a
// for target ABI requirements.

pub fn target() -> Target {
    let mut base = super::android_base::opts();
    base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("-march=armv7-a".to_string());
    Target {
        llvm_target: "armv7-none-linux-android".to_string(),
        pointer_range: 32,
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),
        options: TargetOptions {
            abi: "eabi".to_string(),
            features: "+v7,+thumb-mode,+thumb2,+vfp3,+neon".to_string(),
            max_atomic_width: Some(64),
            ..base
        },
    }
}
