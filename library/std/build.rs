use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target = env::var("TARGET").expect("TARGET was not set");
    if target.contains("freebsd") {
        // There is a special case here for CHERI BSD because it doesn't seem
        // to provide (as of 2023-3-21) versioned symbols, or at least not ones
        // that go back all the way to FreeBSD 11 (`readdir` and `readdir_r`
        // for example).
        // Adding special cases in build scripts to force the right version
        // (FreeBSD 13) is hopefully less error prone than needing to create a
        // script called `freebsd-version` and add it to `PATH` (for `libc`)
        // and set environment variables (for `std`).
        if env::var("RUST_STD_FREEBSD_12_ABI").is_ok() || target.contains("purecap") {
            println!("cargo:rustc-cfg=freebsd12");
        }
    } else if target.contains("linux")
        || target.contains("netbsd")
        || target.contains("dragonfly")
        || target.contains("openbsd")
        || target.contains("solaris")
        || target.contains("illumos")
        || target.contains("apple-darwin")
        || target.contains("apple-ios")
        || target.contains("uwp")
        || target.contains("windows")
        || target.contains("fuchsia")
        || (target.contains("sgx") && target.contains("fortanix"))
        || target.contains("hermit")
        || target.contains("l4re")
        || target.contains("redox")
        || target.contains("haiku")
        || target.contains("vxworks")
        || target.contains("wasm32")
        || target.contains("asmjs")
        || target.contains("espidf")
    {
        // These platforms don't have any special requirements.
    } else {
        // This is for Cargo's build-std support, to mark std as unstable for
        // typically no_std platforms.
        // This covers:
        // - os=none ("bare metal" targets)
        // - mipsel-sony-psp
        // - nvptx64-nvidia-cuda
        // - arch=avr
        // - tvos (aarch64-apple-tvos, x86_64-apple-tvos)
        // - uefi (x86_64-unknown-uefi, i686-unknown-uefi)
        // - JSON targets
        // - Any new targets that have not been explicitly added above.
        println!("cargo:rustc-cfg=feature=\"restricted-std\"");
    }
    println!("cargo:rustc-env=STD_ENV_ARCH={}", env::var("CARGO_CFG_TARGET_ARCH").unwrap());
    println!("cargo:rustc-cfg=backtrace_in_libstd");
}
