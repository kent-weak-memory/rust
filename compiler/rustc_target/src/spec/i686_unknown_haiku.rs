use crate::spec::{LinkerFlavor, StackProbeType, Target};

pub fn target() -> Target {
    let mut base = super::haiku_base::opts();
    base.cpu = "pentium4".to_string();
    base.max_atomic_width = Some(64);
    base.pre_link_args.insert(LinkerFlavor::Gcc, vec!["-m32".to_string()]);
    // don't use probe-stack=inline-asm until rust#83139 and rust#84667 are resolved
    base.stack_probes = StackProbeType::Call;

    Target {
        llvm_target: "i686-unknown-haiku".to_string(),
        pointer_range: 32,
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            f64:32:64-f80:32-n8:16:32-S128"
            .to_string(),
        arch: "x86".to_string(),
        options: base,
    }
}
