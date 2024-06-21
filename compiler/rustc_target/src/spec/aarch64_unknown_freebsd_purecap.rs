use crate::spec::{MergeFunctions, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "aarch64-unknown-freebsd".into(),
        pointer_data_size: 64,
        pointer_memory_size: 128,
        data_layout: "e-m:e-pf200:128:128:128:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-A200-P200-G200".into(),
        arch: "aarch64".into(),
        options: TargetOptions {
            features: "+morello,+c64".into(),
            llvm_abiname: "purecap".into(),
            abi: "purecap".into(), // just for identification via `#[cfg(target_abi=...)]` attributes
            max_atomic_width: Some(128),
            // Atomic pointers are supported and converting to integers
            // invalidates capabilities so we *must* use atomic pointers.
            atomic_pointers_via_integers: false,
            // TODO: figure out why this optimisation causes crashes when building libc.
            merge_functions: MergeFunctions::Disabled,
            ..super::freebsd_base::opts()
        },
    }
}
