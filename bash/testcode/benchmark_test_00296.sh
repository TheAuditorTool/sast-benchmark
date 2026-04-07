#!/bin/bash
write_with_quota() {
    local source="$1"
    local dest="$2"
    local MAX_SIZE_MB=100
    local size_kb
    size_kb=$(du -k "$source" | cut -f1)
    if (( size_kb > MAX_SIZE_MB * 1024 )); then
        echo "File exceeds ${MAX_SIZE_MB}MB limit" >&2
        return 1
    fi
    cp "$source" "$dest"
}
