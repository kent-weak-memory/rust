use crate::spec::{FramePointer, LinkerFlavor, LldFlavor, Target};

pub fn target() -> Target {
    let mut base = super::windows_uwp_gnu_base::opts();
    base.cpu = "pentium4".to_string();
    base.pre_link_args
        .insert(LinkerFlavor::Lld(LldFlavor::Ld), vec!["-m".to_string(), "i386pe".to_string()]);
    base.max_atomic_width = Some(64);
    base.frame_pointer = FramePointer::Always; // Required for backtraces

    // Mark all dynamic libraries and executables as compatible with the larger 4GiB address
    // space available to x86 Windows binaries on x86_64.
    base.pre_link_args
        .entry(LinkerFlavor::Gcc)
        .or_default()
        .push("-Wl,--large-address-aware".to_string());

    Target {
        llvm_target: "i686-pc-windows-gnu".to_string(),
        pointer_range: 32,
        pointer_width: 32,
        data_layout: "e-m:x-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            i64:64-f80:32-n8:16:32-a:0:32-S32"
            .to_string(),
        arch: "x86".to_string(),
        options: base,
    }
}
