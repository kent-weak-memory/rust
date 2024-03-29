use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        // FIXME: this comment below does not seem applicable?
        // It's important we use "gnueabihf" and not "musleabihf" here. LLVM
        // uses it to determine the calling convention and float ABI, and LLVM
        // doesn't support the "musleabihf" value.
        llvm_target: "armv5te-unknown-linux-gnueabi".to_string(),
        pointer_range: 32,
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),
        options: TargetOptions {
            abi: "eabi".to_string(),
            features: "+soft-float,+strict-align".to_string(),
            // Atomic operations provided by compiler-builtins
            max_atomic_width: Some(32),
            mcount: "\u{1}mcount".to_string(),
            has_thumb_interworking: true,
            ..super::linux_musl_base::opts()
        },
    }
}
