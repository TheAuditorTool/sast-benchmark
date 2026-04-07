#!/bin/bash
replace_in_file() {
    local pattern="$1"
    local replacement="$2"
    local file="$3"
    sed -i "s/${pattern}/${replacement}/g" "$file"
}
