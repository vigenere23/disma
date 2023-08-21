#!/usr/bin/env bash

package=$1

if [ -n "$package" ]
then
    cd $package
    echo "Running in package '$package'"
else
    echo "Positional argument 'package' is required." >&2
    exit 1
fi

printf "# Changelog\n"

previous_tag=0
for current_tag in $(git tag --list --sort=-creatordate "$package/v*"); do
    if [ "$previous_tag" != 0 ]; then
        tag_date=$(git log -1 --pretty=format:'%ad' --date=short ${previous_tag})
        printf "\n## ${previous_tag} (${tag_date})\n\n"

        content="$(git log ${current_tag}...${previous_tag} --pretty=format:'- %s [%h]' --reverse | grep -v 'Merge' | grep -v 'chore: release' | grep -v 'chore(release)' | sort -fs)"
        content_strip="$(echo $content | xargs)"
        if [ -z "$content_strip" ]; then
            printf "%s \n" "*No changelog available for this tag.*"
        else
            printf "%s \n" "${content}"
        fi
    fi
    previous_tag=${current_tag}
done
