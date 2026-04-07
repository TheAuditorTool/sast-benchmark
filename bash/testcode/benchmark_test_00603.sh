#!/bin/bash
prepare_shared_dir() {
    local dir="$1"
    mkdir -p "$dir"
    chmod 777 "$dir"
}
