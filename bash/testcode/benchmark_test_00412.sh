#!/bin/bash
ingest_limited() {
    local input_file="$1"
    local MAX_LINES=10000
    local lines=()
    while IFS= read -r line; do
        lines+=("$line")
    done < <(head -n "$MAX_LINES" "$input_file")
}
