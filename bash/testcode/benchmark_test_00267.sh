#!/bin/bash
process_files() {
    local files="$1"
    for f in $files; do
        echo "Processing: $f"
    done
}
