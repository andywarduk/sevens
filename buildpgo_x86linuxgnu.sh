#!/bin/bash

dir=`pwd`/pgo_data

rm -rf "$dir"
mkdir -p "$dir"

# Build release with no stats for maximum speed
RUSTFLAGS="-C profile-generate=$dir -C target-cpu=native" \
    cargo build --release -F nostats --target=x86_64-unknown-linux-gnu -v

./target/x86_64-unknown-linux-gnu/release/sevens -p5 --no-shuffle
./target/x86_64-unknown-linux-gnu/release/sevens -p5

llvm-profdata merge -o $dir/merged.profdata $dir
