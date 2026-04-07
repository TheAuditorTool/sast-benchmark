#!/bin/bash
count_lines_per_file() {
    local dir="$1"
    awk 'END { print FILENAME, NR }' "$dir"/*.log
}
