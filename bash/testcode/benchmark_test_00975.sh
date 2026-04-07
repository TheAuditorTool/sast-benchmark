#!/bin/bash
create_private_dir() {
    local dir="$1"
    mkdir -p "$dir"
    chmod 700 "$dir"
}
