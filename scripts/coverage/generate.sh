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
mode=$2

if [[ -z "$package" ]]; then
    echo "Missing argument 'package'" >&2
    exit 1
elif [[ -z "$mode" ]]; then
    echo "Missing argument 'mode'" >&2
    exit 1
fi

echo "Running in package '$package'"
cd $package

rm -rf ./target/coverage
mkdir -p ./target/coverage/raw

RUSTFLAGS="-C instrument-coverage" \
LLVM_PROFILE_FILE="target/coverage/raw/%p-%m.profraw" \
cargo test

if [[ "$mode" == "ci" ]]; then
    echo "Generating lcov coverage file..."
    compile_coverage lcov ./target/coverage/coverage.info
    echo "Codecov Coverage file outputed to $package/target/coverage/coverage.info"
elif [[ "$mode" == "local" ]]; then
    echo "Generating HTML coverage file..."
    compile_coverage html ./target/coverage/html
    echo "HTML Coverage file outputed to $package/target/coverage/html/index.html"
else
    echo "Invalid argument 'mode'. Supported modes are: 'ci', 'local'." >&2
    exit 1
fi
