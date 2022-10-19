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
    - Stop broken SROA pass being added in `llvm/lib/Transforms/IPO/PassManagerBuilder.cpp`.
    - Name metadata sections as `.rmeta` in `llvm/lib/CodeGen/TargetLoweringObjectFileImpl.cpp`.
	- Prevents compiler emitting badly aligned memcpys in `llvm/lib/CodeGen/TargetLoweringObjectFileImpl.cpp`.

```
./x.py build
./x.py build tools/cargo
```

~~Last tested version of LLVM: `aba2e0847d0dbc33d7292876c2c22c2d610d5359` (2022-07-16)~~

Last tested version of LLVM: `f35a94e96b1c3cc017ca9581ecfeb405ed86508d` (2022-10-19)

getting cargo to build and run programs on CHERI remote host:
`vi path/to/project/repository/.cargo/config`

```
[build]
rustc = "/path/to/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"

[target.aarch64-unknown-freebsd-purecap]
linker = "/path/to/rust/clang-morello.sh"
runner = ["/path/to/rust/test-runner-morello.sh"]
```

testing compiler components on CHERI remote host:

`TEST_DEVICE_ADDR="localhost:1234" ./x.py test --target aarch64-unknown-freebsd-purecap library/core`

benchmarking compiler components on CHERI remote host:

`TEST_DEVICE_ADDR="localhost:1234" ./x.py bench --target aarch64-unknown-freebsd-purecap library/core`

run benchmark manually:
```
cargo test # build binary
path/to/binary --bench
```

# problems:
cargo complains "can't find crate for `core`" when building for CHERI
fix: `./x.py build library/core`

./x.py build complains "Partition::size() const: Assertion `BeginOffset < EndOffset && "Partitions must span some bytes!"' failed." when building std
fix: apply hacks to disable SROA in LLVM
