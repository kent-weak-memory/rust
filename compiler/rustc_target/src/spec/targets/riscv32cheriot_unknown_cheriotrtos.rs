use crate::spec::{
    Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetMetadata, TargetOptions, cvs,
};

pub(crate) fn target() -> Target {
    let abi = "cheriot";
    Target {
        // The below `data_layout` is explicitly specified by the ilp32e ABI in LLVM. See also
        // `options.llvm_abiname`.
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128-pf200:64:64:64:32-A200-P200-G200".into(),
        llvm_target: "riscv32cheriot-unknown-cheriotrtos".into(),
        metadata: TargetMetadata {
            description: Some("CHERIoT RISC-V (RV32E ISA)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_data_size: 32,
        pointer_memrepr_size: 64,
        arch: "riscv32cheriot".into(),

        options: TargetOptions {
            abi: abi.into(),
            linker_flavor: LinkerFlavor::Darwin(Cc::No, Lld::Yes),
            linker: None,
            cpu: "cheriot".into(),
            llvm_abiname: abi.into(),
            max_atomic_width: None,
            atomic_cas: false,
            features: "+32bit,+c,+cap-mode,+e,+m,+xcheri,+zmmul".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            is_like_cheri: true,
            families: cvs!["cheri", "cheriot"],
            os: "cheriotrtos".into(),
            executables: false,
            ..Default::default()
        },
    }
}
