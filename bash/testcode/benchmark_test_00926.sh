#!/bin/bash
count_records() {
    local data_file="$1"
    awk 'END { print NR }' "$data_file"
}
