this is just a quick note of some useful commands I don't want to forget
they might be useful to everyone else too


## LLVM
rebuild LLVM via cheribuild without pulling commits:
`./cheribuild.py --skip-update morello-llvm-native`

getting rustc and other compiler components built for CHERI:
 - write a config.toml based on config.toml.sarah
 - this will need to reference clang-morello.sh wrapper to link
 - use Morello LLVM commit `f35a94e96b1c3cc017ca9581ecfeb405ed86508d` (2021-06-03)
 - ~~We're now using an up-to-date LLVM, subject to some minor hacks being applied~~
 - apply hacks in `llvm.patch`. They are:
    - stop broken SROA pass being added in `llvm/lib/Transforms/IPO/PassManagerBuilder.cpp`.
    - name metadata sections as `.rmeta` in `llvm/lib/CodeGen/TargetLoweringObjectFileImpl.cpp`.
    - prevents compiler emitting badly aligned memcpys in `llvm/lib/CodeGen/TargetLoweringObjectFileImpl.cpp`.
 - you probably need to copy `FileCheck` from `<cheribuild-dir>/build/morello-llvm-project-build/bin/FileCheck` to `<cheribuild-dir>/output/morello-sdk/bin/FileCheck`

```
./x.py build
./x.py build tools/cargo
```

~~Last tested version of LLVM: `aba2e0847d0dbc33d7292876c2c22c2d610d5359` (2022-07-16)~~
Last tested version of LLVM: `f35a94e96b1c3cc017ca9581ecfeb405ed86508d` (2022-10-19)
The version of LLVM needs to also match the version used to build CHERI BSD because changes have been made to the ABI in some versions.
For more information see: https://github.com/CTSRD-CHERI/cheribsd/blob/main/CHERI-UPDATING.md

getting cargo to build and run programs on CHERI remote host:
`vi path/to/project/repository/.cargo/config`

```
[build]
rustc = "/path/to/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"

[target.aarch64-unknown-freebsd-purecap]
linker = "/path/to/rust/clang-morello.sh"
runner = ["/path/to/rust/test-runner-morello.sh"]
```

cargo config can also often be passed on the command line:

`cargo --config 'build.rustc="/path/to/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"'`

testing compiler components on CHERI remote host:

`TEST_DEVICE_ADDR="localhost:1234" ./x.py test --target aarch64-unknown-freebsd-purecap library/core`

benchmarking compiler components on CHERI remote host:

`TEST_DEVICE_ADDR="localhost:1234" ./x.py bench --target aarch64-unknown-freebsd-purecap library/core`

run benchmark manually:
```
cargo test # build binary
path/to/binary --bench
```

find out whether binary or .so is for plain aarch64 or purecap:
```
readelf -h | grep Flags
```
if flags include 0x10000 ELF is for purecap, otherwise plain aarch64

# problems:
cargo complains "can't find crate for `core`" when building for CHERI
fix: `./x.py build library/core`

./x.py build complains "Partition::size() const: Assertion `BeginOffset < EndOffset && "Partitions must span some bytes!"' failed." when building std
fix: apply hacks to disable SROA in LLVM

./x.py build complains about not being able to find std
possible fix: make sure std has been built for host (it may be needed for build scripts)

CHERI BSD build fails with "This should not be included directly. Include <machine/atomic.h>", "do not include this header, use machine/atomic.h"
fix: edit `cheribsd/tools/build/test-includes/Makefile`, add `.SHELL: name=bash path=/bin/bash hasErrCtl=true check="set -e" ignore="set +e" echo="set -v" quiet="set +v" filter="set +v" errFlag=e echoFlag=v newline="'\n'"
explanation:
More recent CHERI BSD makefiles assume the default shell supports `[^_]*.h` globbing, which fails if a system is using - for example - dash, which does not support this extension.
This causes the glob to include files it should not, which generates a large number of broken test files which include things that should not be included.
The `.SHELL` line can be added to the start of the broken makefile to override the default shell (to Bash in the given fix).
This might break other things, but seemed to work in the particular case that motivated this commentary.

./x.py build complains about `FileCheck executable "<cheribuild-dir>/output/morello-sdk/bin/FileCheck" does not exist`
fix: copy `<cheribuild-dir>/build/morello-llvm-project-build/bin/FileCheck` to `<cheribuild-dir>/output/morello-sdk/bin/FileCheck`

strange runtime errors, especially SIGBUS, C programs using `printf("%d\n", <value>)` or similar printing nonsense
fix: make sure you are compiling with an LLVM version with an ABI that matches your build of CHERI BSD.
These sorts of problems seem to indicate ABI mismatch, and the ABI has changed a few times as noted by: https://github.com/CTSRD-CHERI/cheribsd/blob/main/CHERI-UPDATING.md
It might be desirable to use the same version of LLVM to build both programs and the OS to avoid this kind of weirdness.

rustc fails to build with `Relocations in generic ELF (EM: 183)`, `error adding symbols: file in wrong format`, `collect2: error: ld returned 1 exit status` building for purecap
fix: make sure the Morello sysroot and linker are being used, this usually means using the arguments `--target aarch64-unknown-freebsd-purecap -C linker=<rust-repository>/clang-morello.sh`
