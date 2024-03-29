use crate::spec::{Target, TargetOptions};

// This target is for glibc Linux on ARMv7 without NEON or
// thumb-mode. See the thumbv7neon variant for enabling both.

pub fn target() -> Target {
    Target {
        llvm_target: "armv7-unknown-linux-gnueabihf".to_string(),
        pointer_range: 32,
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),
        options: TargetOptions {
            abi: "eabihf".to_string(),
            // Info about features at https://wiki.debian.org/ArmHardFloatPort
            features: "+v7,+vfp3,-d32,+thumb2,-neon".to_string(),
            max_atomic_width: Some(64),
            mcount: "\u{1}__gnu_mcount_nc".to_string(),
            ..super::linux_gnu_base::opts()
        },
    }
}
