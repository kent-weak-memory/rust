use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "aarch64-unknown-freebsd".to_string(),
        pointer_range: 64,
        pointer_width: 128,
        data_layout: "e-m:e-pf200:128:128:128:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-A200-P200-G200".to_string(),
        arch: "aarch64".to_string(),
        options: TargetOptions {
        	features: "+morello,+c64".to_string(),
        	llvm_abiname: "purecap".to_string(),
        	max_atomic_width: Some(128),
        	..super::freebsd_base::opts()
        },
    }
}
