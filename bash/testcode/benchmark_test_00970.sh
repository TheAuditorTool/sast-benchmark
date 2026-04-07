#!/bin/bash
read_manifest() {
    local component="$1"
    local name
    name=$(basename "$component")
    if [[ "$name" != *".."* ]]; then
        cat "/app/manifests/${name}.json"
    fi
}
