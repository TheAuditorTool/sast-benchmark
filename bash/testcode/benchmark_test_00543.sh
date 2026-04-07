#!/bin/bash
create_secure_dir() {
    local dir="$1"
    mkdir "$dir"
    chmod 700 "$dir"
}
