#!/bin/bash
dir=$(dirname ${BASH_SOURCE[0]})
old=$(pwd)

cd $dir
cargo build && RUST_BACKTRACE=full ./target/debug/main
cd $old
