// Generic AArch64 target for bare-metal code - Floating point enabled
//
// Can be used in conjunction with the `target-feature` and
// `target-cpu` compiler flags to opt-in more hardware-specific
// features.
//
// For example, `-C target-cpu=cortex-a53`.

use super::{LinkerFlavor, LldFlavor, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    let opts = TargetOptions {
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        linker: Some("rust-lld".to_owned()),
        features: "+strict-align,+neon,+fp-armv8".to_string(),
        executables: true,
        relocation_model: RelocModel::Static,
        disable_redzone: true,
        max_atomic_width: Some(128),
        panic_strategy: PanicStrategy::Abort,
        ..Default::default()
    };
    Target {
        llvm_target: "aarch64-unknown-none".to_string(),
        pointer_range: 64,
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        options: opts,
    }
}
