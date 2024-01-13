#!/bin/sh

package=$1
args=""

if [ -n "$package" ]
then
    args="-p $package"
    echo "Running in package '$package'"
else
    echo "Running in default location"
fi

echo "Running 'cargo fmt'"
cargo fmt $args --check

echo "Running 'cargo clippy'"
cargo clippy $args -- -A clippy::from_over_into -A clippy::format-collect -D warnings
