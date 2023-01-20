#!/bin/bash

function compile_coverage {
    output_type=$1
    output_path=$2

    echo "Generating $output_type coverage..."

    grcov ./target/coverage/raw \
        --binary-path ../target/debug \
        -s . \
        -t $output_type \
        --ignore-not-existing \
        --ignore "/*" \
        -o $output_path

    if [[ $? -eq 0 ]]; then
        echo "$output_type coverage outputed to $output_path"
        return 0
    else
        echo "Failed to generate $output_type coverage."
        return 1
    fi
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
    compile_coverage lcov ./target/coverage/coverage.info || exit 1
    compile_coverage cobertura ./target/coverage/coverage.xml || exit 1
elif [[ "$mode" == "local" ]]; then
    compile_coverage html ./target/coverage/html || exit 1
else
    echo "Invalid argument 'mode'. Supported modes are: 'ci', 'local'." >&2
    exit 1
fi
