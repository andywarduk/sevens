#!/bin/bash

if [ ! -f ./target/x86_64-unknown-linux-gnu/release/sevens ]
then
    ./build_x86linuxgnu.sh

    if [ $? -ne 0 ]
    then
        exit 1
    fi
fi

./target/x86_64-unknown-linux-gnu/release/sevens $*
