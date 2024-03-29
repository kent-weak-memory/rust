use super::apple_sdk_base::{opts, Arch};
use crate::spec::{FramePointer, Target, TargetOptions};

pub fn target() -> Target {
    // Clang automatically chooses a more specific target based on
    // IPHONEOS_DEPLOYMENT_TARGET.
    // This is required for the target to pick the right
    // MACH-O commands, so we do too.
    let arch = "arm64";
    let llvm_target = super::apple_base::ios_llvm_target(arch);

    Target {
        llvm_target,
        pointer_range: 64,
        pointer_width: 64,
        data_layout: "e-m:o-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        options: TargetOptions {
            features: "+neon,+fp-armv8,+apple-a7".to_string(),
            max_atomic_width: Some(128),
            forces_embed_bitcode: true,
            frame_pointer: FramePointer::NonLeaf,
            // Taken from a clang build on Xcode 11.4.1.
            // These arguments are not actually invoked - they just have
            // to look right to pass App Store validation.
            bitcode_llvm_cmdline: "-triple\0\
                arm64-apple-ios11.0.0\0\
                -emit-obj\0\
                -disable-llvm-passes\0\
                -target-abi\0\
                darwinpcs\0\
                -Os\0"
                .to_string(),
            ..opts("ios", Arch::Arm64)
        },
    }
}
