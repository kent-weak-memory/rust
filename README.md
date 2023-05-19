# The Rust Programming Language (experimental Morello port)

This is a fork of [Rust](https://www.rust-lang.org) which adds experimental support for [Morello](https://www.cl.cam.ac.uk/research/security/ctsrd/cheri/cheri-morello.html).
Morello is an experimental processor architecture that adds [CHERI](https://www.cl.cam.ac.uk/research/security/ctsrd/cheri/) protections to ARM.
This repository contains the compiler, standard library, and documentation.

As of time of writing (2023-05-16), this compiler is functional but somewhat buggy and contains some dirty hacks we haven't fixed yet.
Expect things to break.

Please note that this fork is not built or maintained by the Rust project proper, don't complain to them if you have issues with it, they will not be able to help.
We can be contacted via email or Github issues but don't really have the capacity to offer much support.

## Setup

This fork is intended to be used as a cross compiler.
We have not yet tried running the compiler itself on Morello.

The fork has been used and tested compiling from x86-64 Linux and M1 Mac OS Ventura.
Mac OS requires some additional setup.
Other platforms may or may not work.

Programs can be compiled for Purecap and Hybrid Morello.

### Cheribuild

Cheribuild is required to compile Morello LLVM and CHERI BSD.

Clone it from `https://github.com/CTSRD-CHERI/cheribuild` and follow their setup instructions for your platform.

### Build CHERI BSD

Building libraries and programs requires a build of CHERI BSD to compile against.

Use Cheribuild to clone and build it:
```
cd /path/to/cheribuild
./cheribuild.py -d cheribsd-purecap-morello
```

This will also build Morello LLVM as a dependency of `cheribsd-purecap-morello`

### Patch and Rebuild Morello LLVM

Our compiler requires minor changes to Morello LLVM to function properly.

Checkout a known working version of Morello LLVM and apply patches:
```
cd /path/to/cheribuild-sources-and-output/morello-llvm-project
git fetch --unshallow
git checkout f35a94e96b1c3cc017ca9581ecfeb405ed86508d
git apply /path/to/rust/llvm.patch
```

Rebuild Morello LLVM:
```
cd /path/to/cheribuild
./cheribuild.py --skip-update morello-llvm
```

### Compiler configuration

Rust requires an appropriate `config.toml` to specify targets to build.
Our compiler also requires paths to tools to be configured in `cheri-config.sh`.

Modify `cheri-config.sh` to point at your Cheribuild output directory.
By default this will be `~/cheri/`.

Create a `config.toml`, with contents along the lines of the example below.
Some values will need to be replaced with the correct ones for your environment:

* `/path/to/cheribuild-output-and-source` with the absolute path to your Cheribuild working directory
* `/path/to/rust` with the absolute path to your clone of this compiler repository
* `aarch64-apple-darwin` with the architecture for the system you want to run the compiler on (`aarch64-apple-darwin` for M1 Mac OS, `x86_64-unknown-linux-gnu` for x86-64 Linux)

```toml
profile = "codegen"
changelog-seen = 2

[build]
target = ["aarch64-apple-darwin", "aarch64-unknown-freebsd-purecap"]

[target.aarch64-apple-darwin]
llvm-config = "/path/to/cheribuild-output-and-source/output/morello-sdk/bin/llvm-config"

# This section tells the compiler to use custom scripts for linking Hybrid programs.
[target.aarch64-unknown-freebsd]
cc = "/path/to/rust/clang-freebsd.sh"
cxx = "/path/to/rust/clang++-freebsd.sh"
linker = "/path/to/rust/clang-freebsd.sh"
ar = "/path/to/cheribuild-output-and-source/output/morello-sdk/bin/ar"
ranlib = "/path/to/cheribuild-output-and-source/output/morello-sdk/bin/ranlib"

# This section tells the compiler to use custom scripts for linking Purecap programs.
[target.aarch64-unknown-freebsd-purecap]
cc = "/path/to/rust/clang-morello.sh"
cxx = "/path/to/rust/clang++-morello.sh"
ar = "/path/to/cheribuild-output-and-source/output/morello-sdk/bin/ar"
ranlib = "/path/to/cheribuild-output-and-source/output/morello-sdk/bin/ranlib"
linker = "/path/to/rust/clang-morello.sh"
```

### Compile Rust

Building programs requires a build of the compiler, a build of the Rust standard libraries for the architecture of the machine that will run the compiler (for build scripts), and a build of the standard libraries for Morello.

Build the compiler and standard libraries:
```
cd /path/to/rust
python3 x.py build
```

You may also want to build the package manager Cargo if that is your preferred way to run the compiler.
```
python3 x.py build tools/cargo
```

### Compiling Programs

Programs can be built either with the Rust package manager, Cargo, or manually with just the compiler.

*Cargo*

Assuming you have already built Cargo, it can be run from `/path/to/rust/build/aarch64-apple-darwin/stage1-tools-bin/cargo`.
You will need rustc to be in the environment variable `PATH`, and Cargo must be told to use the correct linker for Morello via a `.cargo/config` configuration file in the top level of the project being built.

Create `.cargo/config` with this contents:
```
[target.aarch64-unknown-freebsd]
linker = "/path/to/rust/clang-freebsd.sh"

[target.aarch64-unknown-freebsd-purecap]
linker = "/path/to/rust/clang-morello.sh"
```

Cargo can then be run like this:
```
PATH="/path/to/rust/build/aarch64-apple-darwin/stage1/bin:$PATH" /path/to/rust/build/aarch64-apple-darwin/stage1-tools-bin/cargo build --target aarch64-unknown-freebsd-purecap
```
The resulting executable will appear at `target/aarch64-unknown-freebsd-purecap/debug/program-name`

*Manual*

The compiler itself can be found in `/path/to/rust/build/aarch64-apple-darwin/stage1/bin/rustc`

It will default to building for the machine it is running on, so building for the current machine looks like this:
```
/path/to/rust/build/aarch64-apple-darwin/stage1/bin/rustc --out-dir /tmp/build -g program.rs
```
The resulting binary will appear at `/tmp/build/program`

To build for Morello Purecap, specify the architecture and linker:
```
/path/to/rust/build/aarch64-apple-darwin/stage1/bin/rustc --out-dir /tmp/build -g --target aarch64-unknown-freebsd-purecap -C linker=/path/to/rust/clang-morello.sh program.rs
```

To build for Morello Hybrid:
```
/path/to/rust/build/aarch64-apple-darwin/stage1/bin/rustc --out-dir /tmp/build -g --target aarch64-unknown-freebsd -C linker=/path/to/rust/clang-freebsd.sh program.rs
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
