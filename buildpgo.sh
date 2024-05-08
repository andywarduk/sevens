#!/bin/bash

# Needs cargo pgo installed - cargo install cargo-pgo

# Build for native CPU
export RUSTFLAGS="-Ctarget-cpu=native"

# Build instrumented binary
echo "============================= Building instrumented binary ==============================="
cargo pgo build -- $* -v

if [ $? -ne 0 ]; then
	exit 1
fi

# Run instrumented binary
echo "============================= Running instrumented binary ==============================="
./target/x86_64-unknown-linux-gnu/release/sevens -p5 --no-shuffle
./target/x86_64-unknown-linux-gnu/release/sevens -p5
./target/x86_64-unknown-linux-gnu/release/sevens -p6

# Build optimised binary
echo "============================= Building optimised binary ==============================="
cargo pgo optimize build -- $*

