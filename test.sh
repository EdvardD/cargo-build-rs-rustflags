#!/bin/bash -e

rm -rf target
cargo build
cat ./target/debug/build/*/output
