this is just a quick note of some useful commands I don't want to forget
they might be useful to everyone else too


## LLVM
rebuild LLVM via cheribuild without pulling commits:
`./cheribuild.py --skip-update morello-llvm-native`

getting rustc and other compiler components built for CHERI:
 - write a config.toml based on config.toml.sarah
 - this will need to reference clang-morello.sh wrapper to link
 - use Morello LLVM commit `671d6dbe2b74525702368edfa086e68f5afadc24`
 - apply hacks in `llvm.patch`. They are:
    - stop broken SROA pass being added in `llvm/lib/Transforms/IPO/PassManagerBuilder.cpp`.
 - you probably need to copy `FileCheck` from `<cheribuild-dir>/build/morello-llvm-project-build/bin/FileCheck` to `<cheribuild-dir>/output/morello-sdk/bin/FileCheck`

```
./x.py build
./x.py build tools/cargo
```

~~Last tested version of LLVM: `aba2e0847d0dbc33d7292876c2c22c2d610d5359` (2022-07-16)~~
~~Last tested version of LLVM: `f35a94e96b1c3cc017ca9581ecfeb405ed86508d` (2022-10-19)~~
Last tested version of LLVM: `671d6dbe2b74525702368edfa086e68f5afadc24` (2024-07-09)
The version of LLVM needs to also match the version used to build CHERI BSD because changes have been made to the ABI in some versions.
For more information see: https://github.com/CTSRD-CHERI/cheribsd/blob/main/CHERI-UPDATING.md

# Cargo
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

# Other Commands
find out whether binary or .so is for plain aarch64 or purecap:
```
readelf -h | grep Flags
```
if flags include 0x10000 ELF is for purecap, otherwise plain aarch64

# Rust code
only build part of a program when compiling for Morello in purecap mode:
```
#![feature(cfg_target_abi)]

#[cfg(not(bootstrap))] // the bootstrap compiler doesn't know about purecap `target_abi`.
#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
{
	// Morello specific code.
}
```

# Types of Test Failure
Mismatch of stderr in compilation test where expected output was `HEX_DUMP` but actual output was `0xHEX_DUMP\n0xHEX_DUMP`:
Many tests normalise stderr so that dumps of `const` data are the same regardless of target word size.
Morello capabilities are wide enough that the normalisation often doesn't work properly.
Fix: broaden the pattern match used for normalisation.
Replace
```
// normalize-stderr-test "([0-9a-f][0-9a-f] |╾─*a(lloc)?[0-9]+(\+[a-z0-9]+)?─*╼ )+ *│.*" -> "HEX_DUMP"
```
with
```
// normalize-stderr-test "(0x[0-9][0-9] │ )?([0-9a-f][0-9a-f] |╾─*a(lloc)?[0-9]+(\+[a-z0-9]+)?─*╼ )+(__ )* *│.*" -> "HEX_DUMP"
// normalize-stderr-test "HEX_DUMP(\n[ \t]+HEX_DUMP)+" -> "HEX_DUMP"
```
Update line numbers in each `*.stderr` file for that test.

Mismatch of stderr in compilation test where layout of `const` dump is different:
```
-          = note: the raw bytes of the constant (size: 16, align: 8) {
-                      ╾───────alloc3────────╼ ╾───────alloc4────────╼ │ ╾──────╼╾──────╼
+          = note: the raw bytes of the constant (size: 32, align: 16) {
+                      0x00 │ ╾───────────────────alloc3────────────────────╼ │ ╾──────────────╼
+                      0x10 │ ╾───────────────────alloc4────────────────────╼ │ ╾──────────────╼
```
Some tests check the layout of pointers in `const` evaluation, which will be different on CHERI targets.
It likely already has separate stderr files for 32 and 64 bit target output.
Fix: add new stderr file for CHERI targets.
Insert this near the top of test:
```
// stderr-per-cheri
```
You will likely find this already there:
```
// stderr-per-bitwidth
```
These tell the build system to check stderr against different stderr files.
For CHERI these will be `test-name.stderr` and `test-name.cheri.stderr`.
For bit width these will be `test-name.32bit.stderr` and `test-name.64bit.stderr`.
When both are used in a test, bit width is first: `test-name.64bit.cheri.stderr`.
You will need to create the new file and make sure its contents are actually correct.
You will also need to update the line numbers in each `*.stderr` file for that test.

Mismatch of stderr in compilation test where `usize` is `transmute()`ed to pointer:
```
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
  --> /home/rust-20211014/tests/ui/consts/const-eval/ub-upvars.rs:7:42
   |
LL |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
   |                                          ^^^^^^^^^^^^^^
   |
   = note: source type: `usize` (64 bits)
   = note: target type: `&u16` (128 bits)
```
Pointer and `usize` are now different sizes on CHERI targets, which violates the expectations of some tests.
`transmute()` is only allowed between types of the same size, so places where it has been used expecting the two to have the same size will now cause errors.
Do not confuse this type of failure with failures caused by `transmute()`ing from pointer to `usize`, which are more complicated to solve.
Fix: in many cases `transmute()` can be replaced with `as`, but take care not to do this in tests where it is `transmute()` that is being tested.
Replace
```
let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
```
with
```
let bad_ref: &'static u16 = unsafe { mem::transmute(0usize as *const u16) };
```

Mismatch of stderr in compilation test caused by debugging output from Morello LLVM:
In the version we are currently using, commit 671d6dbe2b74525702368edfa086e68f5afadc24, Morello LLVM sometimes emits debugging information.
Tests that compare everything from stderr to some expected pattern will fail as result of these extra messages.
Typical output looks like this:
```
DON'T know how to handle   %1695 = insertvalue { i64 addrspace(200)*, i32 } poison, i64 addrspa
ce(200)* %1694, 0DON'T know how to handle   %1124 = insertvalue { i64 addrspace(200)*, i32 } po
ison, i64 addrspace(200)* %1123, 0DON'T know how to handle   %890 = insertvalue { i64 addrspace(200)*, i32 } poison, i64 addrspace(200)* %889, 0DON'T know how to handle   %781 = insertvalue
{ i64 addrspace(200)*, i32 } poison, i64 addrspace(200)* %780, 0DON'T know how to handle   %661 = insertvalue { i64 addrspace(200)*, i32 } poison, i64 addrspace(200)* %660, 0DON'T know how to handle   %464 = insertvalue { i64 addrspace(200)*, i32 } poison, i64 addrspace(200)* %463, 0DON'T know how to handle   %272 = insertvalue { i64 addrspace(200)*, i32 } poison, i64 addrspace(200)* %271, 0DON'T know how to handle   %169 = insertvalue { i64 addrspace(200)*, i32 } poison, i64 addrspace(200)* %168, 0
```
At some point hopefully Morello LLVM will be fixed, or we will use a version with debug logging disabled, but for the moment the easiest way to fix these tests is to add normalisation rules that remove the debugging output.
Fix: add a comment like this to the top of the test:
```
// normalize-stderr-test: "DON'T know how to handle.*\n" -> ""
```
You will also need to update the line numbers in each `*.stderr` file for that test.

Mismatch of stderr in layout test caused by changes to layout representation:
```
1       error: layout_of(std::result::Result<[u32; 0], bool>) = Layout {
-                  size: Size(4 bytes),
+                  data_size: None,
+                  memory_size: Size(4 bytes),
3                  align: AbiAndPrefAlign {
4                      abi: Align(4 bytes),
5                      pref: $PREF_ALIGN,

6                  },
7                  abi: Aggregate {
+                      metadata: false,
8                      sized: true,
9                  },
```
The compiler's internal layout `struct` has been changed in the following ways:
- `size` has been replaced by `data_size` and `memory_size`
- `Aggregate` has an additional field `metadata`
Fix: add the missing fields based on the types being analysed in the test.
This requires some care.
`data_size` contains an optional size, and will be `None` where it is not reasonable for a type to have a single region containing data which is not capability metadata.
`memory_size` is effectively a replacement for `size`, and contains a plain size representing the overall size of the type.
`metadata` is a boolean, and should be `true` for types that contain capabilities.

# Problems:
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
