#!/bin/bash

package=$1

if [[ -z "$package" ]]; then
    echo "Missing argument 'package'" >&2
    exit 1
fi

echo "Running in package '$package'"
cd $package

rm -rf ./target/coverage
mkdir -p ./target/coverage

CARGO_INCREMENTAL=0 \
RUSTFLAGS="-C instrument-coverage" \
LLVM_PROFILE_FILE="target/coverage/%p-%m.profraw" \
cargo test

grcov ./target/coverage \
    --binary-path ../target/debug \
    -s . \
    -t html \
    --ignore-not-existing \
    -o ./target/coverage/html

echo "Coverage file outputed to $package/target/coverage/html/index.html"
