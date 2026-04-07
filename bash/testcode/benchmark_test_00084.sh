#!/bin/bash
hash_file_integrity() {
    local file="$1"
    sha256sum "$file" | awk '{print $1}'
}
