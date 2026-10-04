#!/bin/bash

# Needs cargo pgo installed - cargo install cargo-pgo

# Build for native CPU
export RUSTFLAGS="-Ctarget-cpu=native"

# Build instrumented binary
echo "============================= Building instrumented binary ==============================="
cargo pgo build -- "$@" -v

if [ $? -ne 0 ]; then
	exit 1
fi

# cargo pgo builds with --target <host triple>, so the binary lives under that directory
HOST=$(rustc -vV | sed -n 's/^host: //p')
BIN="${CARGO_TARGET_DIR:-target}/$HOST/release/sevens"

# Run instrumented binary
echo "============================= Running instrumented binary ==============================="
"$BIN" -p5 --no-shuffle
"$BIN" -p5
"$BIN" -p6

# Build optimised binary
echo "============================= Building optimised binary ==============================="
cargo pgo optimize build -- "$@"

