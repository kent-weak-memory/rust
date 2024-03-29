use crate::abi::Endian;
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "mipsisa64r6-unknown-linux-gnuabi64".to_string(),
        pointer_range: 64,
        pointer_width: 64,
        data_layout: "E-m:e-i8:8:32-i16:16:32-i64:64-n32:64-S128".to_string(),
        arch: "mips64".to_string(),
        options: TargetOptions {
            abi: "abi64".to_string(),
            endian: Endian::Big,
            // NOTE(mips64r6) matches C toolchain
            cpu: "mips64r6".to_string(),
            features: "+mips64r6".to_string(),
            max_atomic_width: Some(64),
            mcount: "_mcount".to_string(),

            ..super::linux_gnu_base::opts()
        },
    }
}
