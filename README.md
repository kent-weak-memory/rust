# The Rust Programming Language (experimental Morello port)

This is a fork of [Rust](https://www.rust-lang.org) which adds experimental support for [Morello](https://www.cl.cam.ac.uk/research/security/ctsrd/cheri/cheri-morello.html).
Morello is an experimental processor architecture that adds [CHERI](https://www.cl.cam.ac.uk/research/security/ctsrd/cheri/) protections to ARM (AArch64).
This repository contains the compiler, standard library, and documentation.

As of time of writing (2024-01-12), this compiler should be fully functional, but has bugs and contains some dirty hacks we haven't fixed yet.
Expect things to break.

This fork is not built or maintained by the Rust project proper, please don't complain to them if you have issues with it, they will not be able to help.
We can be contacted via email or GitHub issues but don't really have the capacity to offer much support.

## Setup

This fork is intended to be used as a cross compiler.
We have not yet tried running the compiler itself on Morello.

The fork has been used and tested compiling from x86-64 Linux and M1 Mac OS Ventura.
Mac OS requires some additional setup.
Other platforms may or may not work.

Programs can be compiled for CheriBSD running on Morello, and other targets already supported by Rust.

There are two ways to install the compiler:
* download a pre-built binary (if one is available for your platform)
* build the compiler from source

## Pre-built Binaries

See <https://github.com/kent-weak-memory/rust/releases> for available builds.
Releases as of January 2024 should work straight from the directory they are extracted into, and include:
* the Rust compiler, `rustc`
* the Rust package manager, `cargo`
* the Rust standard library for Morello FreeBSD, AArch64 FreeBSD, and the platform the release is for
* supporting system libraries for Morello and AArch64
* extra tools for Morello and AArch64
* a simple example program

Note that releases from before January 2024 had a different structure and content, and will be more awkward to use.

### Compiling Programs

Programs can be built either with the Rust package manager, Cargo, or manually with just the compiler.

*Cargo*

Cargo must be told to use the correct compiler and linker for Morello via a `.cargo/config` configuration file in either your home directory or the top level directory of the project you are trying to build.
See the example in `example_project`.

The following sets the Rust compiler for all platforms, and the linker for Morello and AArch64, you will need to modify the paths for your installation:
```
[build]
rustc = "/path/to/release/bin/rustc"

[target.aarch64-unknown-freebsd]
linker = "/path/to/release/bin/clang-freebsd.sh"

[target.aarch64-unknown-freebsd-purecap]
linker = "/path/to/release/bin/clang-morello.sh"
```

The following commands, run from the root directory of your project, will build it for Morello, AArch64, and the local machine's architecture respectively:
```
/path/to/release/bin/cargo build --target aarch64-unknown-freebsd-purecap
/path/to/release/bin/cargo build --target aarch64-unknown-freebsd
/path/to/release/bin/cargo build
```

The resulting executable will appear at `target/aarch64-unknown-freebsd-purecap/debug/program-name` for Morello, and similarly named directories for other targets.

*Manual*

To build for Morello Purecap:
```
/path/to/release/bin/rustc --out-dir /tmp/build -g --target aarch64-unknown-freebsd-purecap -C linker=/path/to/release/bin/clang-morello.sh program.rs
```

The resulting binary will appear at `/tmp/build/program`

To build for AArch64:
```
/path/to/release/bin/rustc --out-dir /tmp/build -g --target aarch64-unknown-freebsd -C linker=/path/to/release/bin/clang-freebsd.sh program.rs
```

To build for the architecture of the local machine:
```
/path/to/release/bin/rustc --out-dir /tmp/build -g program.rs
```

## Building from Source

At a high level the steps to build from source are:
* compile Morello LLVM
* compile CheriBSD
* compile Rust compiler and libraries
* compile additional Rust tools

### Cheribuild

Cheribuild is required to compile Morello LLVM and CHERI BSD.

Clone it from `https://github.com/CTSRD-CHERI/cheribuild` and follow their setup instructions for your platform.

### CHERI BSD (and Morello LLVM)

Building libraries and programs requires a build of Morello LLVM as a compiler backend, and a build of CHERI BSD to compile against.
Cheribuild will automatically compile Morello LLVM as a dependency for building CHERI BSD, so we can do both in one go:
```
cd /path/to/cheribuild
./cheribuild.py -d cheribsd-morello-purecap
```

Note that this process will download large amounts of source code from the internet.

### Clone Compiler Repository

If you have not done so already, clone this repository so it is available for later steps.
Be aware that the complete repository is very large.
Using a shallow clone can save time and storage if you don't need the entire commit history:
```
git clone --depth=1 https://github.com/kent-weak-memory/rust.git
```

### Patch and Rebuild Morello LLVM

Our compiler requires minor changes to Morello LLVM to function properly.

Checkout a known working version of Morello LLVM and apply patches:
```
cd /path/to/cheribuild-working-directory/morello-llvm-project
git fetch --unshallow
git checkout f35a94e96b1c3cc017ca9581ecfeb405ed86508d
git apply /path/to/rust-repository/llvm.patch
```

Rebuild Morello LLVM:
```
cd /path/to/cheribuild
./cheribuild.py --skip-update morello-llvm
```

### Compiler configuration

Rust requires a `config.toml` which provides details of how to build the compiler.
In addition, our fork also requires paths to tools to be set in `cheri-config.sh`.

Modify `cheri-config.sh` to point at your Cheribuild output directory.
By default this will be `~/cheri/` on UNIX-like systems.

Create a `config.toml`, with contents along the lines of the example below.
Some values will need to be replaced with the correct ones for your environment:

* `/path/to/cheribuild-working-directory` replace with the absolute path to your Cheribuild working directory
* `/path/to/rust-repository` replace with the absolute path to your clone of this compiler repository
* `aarch64-apple-darwin` replace with the architecture for the system you want to run the compiler on (`aarch64-apple-darwin` for M1 Mac OS, `x86_64-unknown-linux-gnu` for x86-64 Linux)

```toml
profile = "codegen"
changelog-seen = 2

[build]
target = ["aarch64-apple-darwin", "aarch64-unknown-freebsd-purecap", "aarch64-unknown-freebsd"]

[target.aarch64-apple-darwin]
llvm-config = "/path/to/cheribuild-working-directory/output/morello-sdk/bin/llvm-config"

# This section tells the compiler to use custom scripts for linking AArch64 programs.
[target.aarch64-unknown-freebsd]
cc = "/path/to/rust-repository/clang-freebsd.sh"
cxx = "/path/to/rust-repository/clang++-freebsd.sh"
linker = "/path/to/rust-repository/clang-freebsd.sh"
ar = "/path/to/cheribuild-working-directory/output/morello-sdk/bin/ar"
ranlib = "/path/to/cheribuild-working-directory/output/morello-sdk/bin/ranlib"

# This section tells the compiler to use custom scripts for linking Purecap programs.
[target.aarch64-unknown-freebsd-purecap]
cc = "/path/to/rust-repository/clang-morello.sh"
cxx = "/path/to/rust-repository/clang++-morello.sh"
ar = "/path/to/cheribuild-working-directory/output/morello-sdk/bin/ar"
ranlib = "/path/to/cheribuild-working-directory/output/morello-sdk/bin/ranlib"
linker = "/path/to/rust-repository/clang-morello.sh"
```

### Compile Rust

Building programs requires a build of the compiler, a build of the Rust standard libraries for the architecture of the machine that will run the compiler (for build scripts), and a build of the standard libraries for the target machine.

Build the compiler and standard libraries:
```
cd /path/to/rust-repository
python3 x.py build
```

Note that this process will download large amounts of source code from the internet.

You may also want to build the package manager Cargo:
```
python3 x.py build tools/cargo
```

### Compiling Programs

Programs can be built either with the Rust package manager, Cargo, or manually with just the compiler.

*Cargo*

Assuming you have already built Cargo, it can be run from `/path/to/rust-repository/build/aarch64-apple-darwin/stage1-tools-bin/cargo` (replacing `aarch64-apple-darwin` if your machine has a different architecture).
Cargo must be told to use the correct compiler and linker for Morello via a `.cargo/config` configuration file in either your home directory or the top level directory of the project you are trying to build.

The following sets the Rust compiler for all platforms, and the linker for Morello and AArch64, you will need to modify the paths for your installation and architecture:
```
[build]
rustc = "/path/to/rust-repository/build/aarch64-apple-darwin/stage1/bin/rustc"

[target.aarch64-unknown-freebsd]
linker = "/path/to/rust-repository/clang-freebsd.sh"

[target.aarch64-unknown-freebsd-purecap]
linker = "/path/to/rust-repository/clang-morello.sh"
```

The following commands, run from the root directory of your project, will build it for Morello, AArch64, and the local machine's architecture respectively:
```
/path/to/rust-repository/build/aarch64-apple-darwin/stage1-tools-bin/cargo build --target aarch64-unknown-freebsd-purecap
/path/to/rust-repository/build/aarch64-apple-darwin/stage1-tools-bin/cargo build --target aarch64-unknown-freebsd
/path/to/rust-repository/build/aarch64-apple-darwin/stage1-tools-bin/cargo build
```

The resulting executable will appear at `target/aarch64-unknown-freebsd-purecap/debug/program-name` for Morello, and similarly named directories for other targets.

*Manual*

The compiler itself can be found in `/path/to/rust-repository/build/aarch64-apple-darwin/stage1/bin/rustc`

It will default to building for the machine it is running on, so building for the current machine looks like this:
```
/path/to/rust-repository/build/aarch64-apple-darwin/stage1/bin/rustc --out-dir /tmp/build -g program.rs
```
The resulting binary will appear at `/tmp/build/program`

To build for Morello Purecap, specify the architecture and linker:
```
/path/to/rust-repository/build/aarch64-apple-darwin/stage1/bin/rustc --out-dir /tmp/build -g --target aarch64-unknown-freebsd-purecap -C linker=/path/to/rust-repository/clang-morello.sh program.rs
```

To build for AArch64:
```
/path/to/rust-repository/build/aarch64-apple-darwin/stage1/bin/rustc --out-dir /tmp/build -g --target aarch64-unknown-freebsd -C linker=/path/to/rust-repository/clang-freebsd.sh program.rs
```

### Mac OS Ventura

There are a couple of problems on Mac OS.

*libarchive*

Firstly, there's a packaging issue with upstream libarchive, and the brew install doesn't work perfectly.
To fix this remove `Requires.private: iconv` from `/opt/homebrew/opt/libarchive/lib/pkgconfig/libarchive.pc`.
You might need to do `chmod +w` first.
There are issues tracking this [here](https://github.com/Homebrew/homebrew-core/issues/120526), and [here](https://github.com/CTSRD-CHERI/cheribuild/issues/340).

*failed to `installworld`*

There's some weird interaction between a new feature in Mac OS 13 (Ventura) and the Cheribuild script.
You need to disable SIP by following the Apple instructions [here](https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection).
There is an issue tracking this [here](https://github.com/CTSRD-CHERI/cheribuild/issues/339).

## License

Rust is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
