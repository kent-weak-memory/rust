#!/bin/bash

source $(dirname "$0")/../../../cheri-config.sh

mkdir -p /tmp/build || exit 1
gcc -Wall -Wextra -O2 main.c -o /tmp/build/test-server-x86 || exit 1
$CHERI_HOME/output/morello-sdk/bin/clang \
	-fuse-ld=lld \
	-target aarch64-unknown-freebsd13.0 \
	--sysroot="$CHERI_HOME/output/rootfs-morello-purecap" \
	-B"$CHERI_HOME/output/rootfs-morello-purecap/usr/bin" \
	-pipe \
	-march=morello+c64 -mabi=purecap \
	-femulated-tls -fno-common -fPIE -nobuiltininc \
	-std=gnu99 -Wall -Wextra -O2 \
	main.c -o "/tmp/build/test-server-cheri"
