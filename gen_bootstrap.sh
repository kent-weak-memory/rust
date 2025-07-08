#!/bin/bash

FILE="bootstrap.toml"
HOST_TRIPLE=$(rustc -vV | sed -n 's|host: ||p')

if [ -e "$FILE" ]; then
  echo "$FILE already exists!"
  exit 1
else
cat > "$FILE" <<- EOF

# See bootstrap.example.toml for documentation of available options
#
profile = "compiler"  # Includes one of the default files in src/bootstrap/defaults
change-id = 140732

[rust]
#channel = "nightly"
#codegen-backends = ["llvm"]
#debug = true
#debuginfo-level = 2
std-features = ["compiler-builtins-mem"]

[llvm]
download-ci-llvm = false

[target.$HOST_TRIPLE]
llvm-config = "$CHERIOT_SYSROOT_DIR/bin/llvm-config"

[target.riscv32cheriot-unknown-cheriotrtos]
llvm-config = "$CHERIOT_SYSROOT_DIR/bin/llvm-config"
EOF
fi
