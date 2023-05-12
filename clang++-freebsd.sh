#!/bin/bash
# Quick and dirty wrapper to call the right clang with the right target and system library path.
# I'm not sure why the build system doesn't do this itself.
# See config.toml.sarah for more information.
source $(dirname "$0")/cheri-config.sh

$CHERI_HOME/output/morello-sdk/bin/clang++ --sysroot $CHERI_HOME/output/rootfs-morello-purecap/ -target aarch64-unknown-freebsd "$@"
