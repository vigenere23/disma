#!/bin/sh

get_current_version() {
    while read -r line;
    do
        filtered_line=$(echo $line | sed -r 's/^version = \"(.+)\"$/\1/')
        if [ "$line" != "$filtered_line" ]
        then
            echo $filtered_line
            exit 0
        fi
    done < ./Cargo.toml

    exit 1
}

get_remote_version() {
    package=$(echo $PWD | sed -r 's/^.*\/(.+)$/\1/')
    dependency_line=$(cargo search --limit 1 "$package") || exit
    version=$(echo $dependency_line | sed -r 's/^.+ = \"(.+)\".*$/\1/')

    echo $version

    exit 0
}

current_version=$(get_current_version) || exit
remote_version=$(get_remote_version) || exit

echo "Current version: $current_version"
echo "Remote version: $remote_version"

if [ "$current_version" != "$remote_version" ]
then
    cargo publish --dry-run # TODO remove --dry-run
else
    echo "No need to push new version. Skipping."
fi
