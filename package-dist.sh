#!/bin/sh
# Repacks archives created by `x.py dist` into one single release tarball.
# This is intended to make it easy to explain to users how to use the release.
# The dist tarballs include install scripts, but this is more faff to use, less clear for users, and becomes a nuisance for quickly trying out the release.
#
# Make sure you have ~2GB of space free in /tmp and have built the dist tarballs this script needs.
# It doesn't need everything, so some errors from `x.py dist` can be ignored, while others will break the script.

if [ $# -ne 2 ]; then
	echo "Usage: $0 rust_repository_path cheribuild_working_directory_path"
	exit 1
fi

RUST_PATH="$1"
CHERIBUILD_PATH="$2"

VERSION='1.56.0'
HOST='x86_64-unknown-linux-gnu'
PACKAGES="
	rustc-$VERSION-dev-$HOST/rustc
	rust-std-$VERSION-dev-$HOST/rust-std-$HOST
	rust-std-$VERSION-dev-aarch64-unknown-freebsd/rust-std-aarch64-unknown-freebsd
	rust-std-$VERSION-dev-aarch64-unknown-freebsd-purecap/rust-std-aarch64-unknown-freebsd-purecap
	cargo-$VERSION-dev-$HOST/cargo
"
TOOLS='ar clang clang++ ld nm objcopy objdump ranlib readelf strip'

mkdir -p "/tmp/rust-build-$VERSION" || exit 1
cd "/tmp/rust-build-$VERSION" || exit 1

# Copy Rust components.
for PACKAGE in $PACKAGES; do
	NAME="${PACKAGE%/*}"
	tar xf "$RUST_PATH/build/dist/$NAME.tar.gz" || exit 1
	rm "$PACKAGE/manifest.in" || exit 1
	cp -nr "$PACKAGE/"* ./ || exit 1
	rm -r "$NAME" || exit 1
done

cp "$RUST_PATH/clang-morello-release.sh" bin/clang-morello.sh || exit
cp "$RUST_PATH/clang-freebsd-release.sh" bin/clang-freebsd.sh || exit

# Copy LLVM components.
for TOOL in $TOOLS; do
	cp -n "$CHERIBUILD_PATH/output/morello-sdk/bin/$TOOL" bin/ || exit 1
done
mkdir rootfs-morello-purecap || exit 1
cp -nr "$CHERIBUILD_PATH/output/rootfs-morello-purecap/lib" rootfs-morello-purecap/ || exit 1
# cp -nr "$CHERIBUILD_PATH/output/rootfs-morello-purecap/lib64c" rootfs-morello-purecap/ || exit 1
mkdir rootfs-morello-purecap/usr || exit 1
cp -nr "$CHERIBUILD_PATH/output/rootfs-morello-purecap/usr/lib" rootfs-morello-purecap/usr/ || exit 1
cp -nr "$CHERIBUILD_PATH/output/rootfs-morello-purecap/usr/lib64" rootfs-morello-purecap/usr/ || exit 1
# cp -nr "$CHERIBUILD_PATH/output/rootfs-morello-purecap/usr/lib64c" rootfs-morello-purecap/usr/ || exit 1

# Tidy up randomly write-protected libraries, it makes cleaning up a failed
# packaging attempt a bit tedious.
chmod +w -R rootfs-morello-purecap || exit 1

# Example programs.
cat >example.rs <<EOF
fn main() {
	let mut iter = std::env::args();
	let program = iter.next();
	let path = iter.next();
	if path.is_none() || iter.next().is_some() {
		println!("Usage: {} file", program.as_ref().map(String::as_str).unwrap_or("<program>"));
		println!("Reads file, parses it as UTF-8, and prints the result.");
		return;
	}
	let path = path.unwrap();

	match std::fs::read_to_string(path) {
		Ok(data) => print!("{}", data),
		Err(error) => eprintln!("ERROR: {}", error),
	}
}
EOF
mkdir example_project || exit 1
mkdir example_project/.cargo || exit 1
mkdir example_project/src || exit 1
cp -n example.rs example_project/src/main.rs || exit 1
cat >example_project/Cargo.toml <<EOF
[package]
name = "example_project"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF
cat >example_project/.cargo/config.toml <<EOF
[build]
rustc = "/path/to/installation/bin/rustc"

[target.aarch64-unknown-freebsd]
linker = "/path/to/installation/bin/clang-freebsd.sh"

[target.aarch64-unknown-freebsd-purecap]
linker = "/path/to/installation/bin/clang-morello.sh"
EOF

cd .. || exit 1
tar czf "rust-build-$VERSION.tar.gz" "rust-build-$VERSION" || exit 1
