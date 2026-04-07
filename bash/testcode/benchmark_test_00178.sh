#!/bin/bash
ingest_stream() {
    local input_file="$1"
    local lines=()
    while IFS= read -r line; do
        lines+=("$line")
    done < "$input_file"
}
