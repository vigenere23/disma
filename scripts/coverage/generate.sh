#!/bin/bash

function compile_coverage {
    output_type=$1
    output_path=$2

    grcov ./target/coverage/raw \
        --binary-path ../target/debug \
        -s . \
        -t $output_type \
        --ignore-not-existing \
        --ignore "/*" \
        -o $output_path
}

package=$1

if [[ -z "$package" ]]; then
    echo "Missing argument 'package'" >&2
    exit 1
fi

echo "Running in package '$package'"
cd $package

rm -rf ./target/coverage
mkdir -p ./target/coverage/raw

RUSTFLAGS="-C instrument-coverage" \
LLVM_PROFILE_FILE="target/coverage/raw/%p-%m.profraw" \
cargo test

echo "Generating lcov coverage file..."
compile_coverage lcov ./target/coverage/coverage.txt
echo "Codecov Coverage file outputed to $package/target/coverage/coverage.txt"

echo "Generating HTML coverage file..."
compile_coverage html ./target/coverage/html
echo "HTML Coverage file outputed to $package/target/coverage/html/index.html"
