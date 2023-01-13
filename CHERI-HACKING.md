# CHERI-HACKING
This is designed to serve as a guide for getting up and running with the Rust compiler for Morello.
This is hopefully complete, but please let us know if there are bugs by email or github issue.
We have run the compiler on Linux and macOS Ventura, there are some special requirements for macOS though.
See below.

# Get cheribuild

Clone the cheribuild project from https://github.com/CTSRD-CHERI/cheribuild.
Follow their setup instructions for your platform.

# Build CHERIBSD

You will need to build CHERIBSD for Purecap Morello, and FreeBSD for Aarch64.

```
./cheribuild.py -d cheribsd-purecap-morello
./cheribuild.py -d freebsd-aarch64
```

This will also build LLVM as a dependent of `cheribsd-purecap-morello`

# Roll-back and patch LLVM

```
cd ~/cheri/morello-llvm-project
git fetch --unshallow
git checkout f35a94e96b1c3cc017ca9581ecfeb405ed86508d
git apply /path/to/rust/llvm.patch
cd /path/to/cheribuild
./cheribuild.py --skip-update morello-llvm
```

# Set-up a Config.toml and configure cheri-config.sh

Modify `cheri-config.sh` to point at your cheri build output.
By default this will be in `~/cheri/`.

Create a config.toml, with contents along these lines:

```toml
profile = "codegen"
changelog-seen = 2

[build]
target = ["aarch64-apple-darwin", "aarch64-unknown-freebsd-purecap"]

# Replace with whatever architecture you're building and running the compiler from.
[target.aarch64-apple-darwin]
llvm-config = "/Users/simon/cheri/output/morello-sdk/bin/llvm-config"

[target.aarch64-unknown-freebsd]
cc = "/Users/simon/github/rust/clang-freebsd.sh"
cxx = "/Users/simon/github/rust/clang++-freebsd.sh"
linker = "/Users/simon/github/rust/clang-freebsd.sh"
ar = "/Users/simon/cheri/output/morello-sdk/bin/ar"
ranlib = "/Users/simon/cheri/output/morello-sdk/bin/ranlib"

[target.aarch64-unknown-freebsd-purecap]
cc = "/Users/simon/github/rust/clang-morello.sh"
cxx = "/Users/simon/github/rust/clang++-morello.sh"
ar = "/Users/simon/cheri/output/morello-sdk/bin/ar"
ranlib = "/Users/simon/cheri/output/morello-sdk/bin/ranlib"
linker = "/Users/simon/github/rust/clang-morello.sh"
```

You will have to modify the paths and targets to match your system.

# Build Rust

You can now build rust using `python3 x.py build`.

# Notes

## macOS

There are a couple of problems on macOS. 

### libarchive
Firstly, there's a packaging issue with upstream libarchive, and the brew install doesn't work perfectly.
To fix this remove `Requires.private: iconv` from `/opt/homebrew/opt/libarchive/lib/pkgconfig/libarchive.pc`.
You might need to do `chmod +w` first.
There are issues tracking this [here](https://github.com/Homebrew/homebrew-core/issues/120526), and [here](https://github.com/CTSRD-CHERI/cheribuild/issues/340).

### Failed to `installworld`
There's some weird interaction between a new feature in macOS 13 (Ventura) and the cheribuild script.
You need to disable SIP by following the Apple instructions [here](https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection).
There is an issue tracking this [here](https://github.com/CTSRD-CHERI/cheribuild/issues/339).
