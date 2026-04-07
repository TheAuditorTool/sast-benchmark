#!/bin/bash
process_file_lines() {
    local user_file="$1"
    while IFS= read -r line; do
        process_line "$line"
    done < <(head -n 10000 "$user_file")
}
