#!/bin/bash
# Quick and dirty wrapper to call the right clang with the right target and system library path.

ROOT_RELATIVE="$(dirname $0)/.."
ROOT="$(cd "$ROOT_RELATIVE" && pwd -P)"

"$ROOT/bin/clang" --sysroot "$ROOT/rootfs-morello-purecap/" -target aarch64-unknown-freebsd -march=morello+c64 -mabi=purecap "$@"
