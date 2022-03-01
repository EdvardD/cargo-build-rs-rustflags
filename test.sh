#!/bin/bash -e

rustup show | head -n1
rm -rf target
cargo build
cat ./target/debug/build/*/output
