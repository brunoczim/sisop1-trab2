#!/usr/bin/env sh

set -e

rm -f output.csv

cargo build
time cargo run -- -o output.csv -m debug

cargo build --release
time cargo run --release -- -o output.csv -m release
