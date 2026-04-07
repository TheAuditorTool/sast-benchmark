#!/bin/bash
compute_offset() {
    local input="$1"
    if [[ ! "$input" =~ ^[0-9]+$ ]]; then
        echo "Invalid number" >&2
        return 1
    fi
    local result=$(( input + 1 ))
    echo "$result"
}
