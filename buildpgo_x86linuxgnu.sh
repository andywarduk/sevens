#!/bin/bash

dir=`pwd`/pgo_data

mkdir -p $dir

RUSTFLAGS="-Cprofile-generate=$dir" \
    cargo build --release --target=x86_64-unknown-linux-gnu

./target/x86_64-unknown-linux-gnu/release/sevens -p5 --no-shuffle
./target/x86_64-unknown-linux-gnu/release/sevens -p5

llvm-profdata merge -o $dir/merged.profdata $dir
