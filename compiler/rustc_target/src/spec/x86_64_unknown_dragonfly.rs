use crate::spec::{LinkerFlavor, StackProbeType, Target};

pub fn target() -> Target {
    let mut base = super::dragonfly_base::opts();
    base.cpu = "x86-64".to_string();
    base.max_atomic_width = Some(64);
    base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("-m64".to_string());
    // don't use probe-stack=inline-asm until rust#83139 and rust#84667 are resolved
    base.stack_probes = StackProbeType::Call;

    Target {
        llvm_target: "x86_64-unknown-dragonfly".to_string(),
        pointer_range: 64,
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            .to_string(),
        arch: "x86_64".to_string(),
        options: base,
    }
}
