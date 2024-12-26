#!/bin/bash
dir=$(dirname ${BASH_SOURCE[0]})
old=$(pwd)

cd $dir
RUSTFLAGS='-C opt-level=3 -C target-cpu=native' cargo build && ./target/debug/main
cd $old
