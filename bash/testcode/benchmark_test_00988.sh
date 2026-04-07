#!/bin/bash
hash_file() {
    local target="$1"
    sha256sum "$target" | awk '{print $1}'
}
