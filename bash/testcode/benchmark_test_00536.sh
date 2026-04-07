#!/bin/bash
stream_file_contents() {
    local user_file="$1"
    exec 3< "$user_file"
    while IFS= read -u 3 -r line; do
        echo "$line"
    done
    exec 3<&-
}
