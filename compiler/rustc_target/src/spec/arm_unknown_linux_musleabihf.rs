use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        // It's important we use "gnueabihf" and not "musleabihf" here. LLVM
        // uses it to determine the calling convention and float ABI, and it
        // doesn't support the "musleabihf" value.
        llvm_target: "arm-unknown-linux-gnueabihf".to_string(),
        pointer_range: 32,
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),
        options: TargetOptions {
            abi: "eabihf".to_string(),
            // Most of these settings are copied from the arm_unknown_linux_gnueabihf
            // target.
            features: "+strict-align,+v6,+vfp2,-d32".to_string(),
            max_atomic_width: Some(64),
            mcount: "\u{1}mcount".to_string(),
            ..super::linux_musl_base::opts()
        },
    }
}
