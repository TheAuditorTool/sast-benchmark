#!/bin/bash
read_validated_path() {
    local path="$1"
    local resolved
    resolved=$(readlink -f "$path")
    if [[ "$resolved" != /safe/prefix/* ]]; then
        echo "Invalid path" >&2; return 1
    fi
    cat "$resolved"
}
