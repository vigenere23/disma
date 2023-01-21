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

package=$1

cd $package
echo $(get_current_version)
