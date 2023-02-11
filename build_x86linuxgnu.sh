#!/bin/bash

dir=`pwd`/pgo_data

if [ ! -f $dir/merged.profdata ]
then
    ./buildpgo_x86linuxgnu.sh
fi

# Build release with no stats for maximum speed
RUSTFLAGS="-Cprofile-use=$dir/merged.profdata -Cllvm-args=-pgo-warn-missing-function" \
    cargo build --release -F nostats --target=x86_64-unknown-linux-gnu
