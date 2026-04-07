#!/bin/bash
process_files() {
    local -a files=("$@")
    for f in "${files[@]}"; do
        echo "Processing: $f"
    done
}
