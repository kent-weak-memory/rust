use crate::abi::Endian;
use crate::spec::{LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::vxworks_base::opts();
    base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("-m32".to_string());
    base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("--secure-plt".to_string());
    base.max_atomic_width = Some(32);

    Target {
        llvm_target: "powerpc-unknown-linux-gnu".to_string(),
        pointer_range: 32,
        pointer_width: 32,
        data_layout: "E-m:e-p:32:32-i64:64-n32".to_string(),
        arch: "powerpc".to_string(),
        options: TargetOptions { endian: Endian::Big, features: "+secure-plt".to_string(), ..base },
    }
}
