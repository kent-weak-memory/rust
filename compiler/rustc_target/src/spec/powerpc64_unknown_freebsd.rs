use crate::abi::Endian;
use crate::spec::{LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::freebsd_base::opts();
    base.cpu = "ppc64".to_string();
    base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("-m64".to_string());
    base.max_atomic_width = Some(64);

    Target {
        llvm_target: "powerpc64-unknown-freebsd".to_string(),
        pointer_range: 64,
        pointer_width: 64,
        data_layout: "E-m:e-i64:64-n32:64".to_string(),
        arch: "powerpc64".to_string(),
        options: TargetOptions { endian: Endian::Big, mcount: "_mcount".to_string(), ..base },
    }
}
