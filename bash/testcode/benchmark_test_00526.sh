#!/bin/bash
deduplicate_cache_files() {
    local file_a="$1"
    local file_b="$2"
    local hash_a hash_b
    hash_a=$(md5sum "$file_a" | awk '{print $1}')
    hash_b=$(md5sum "$file_b" | awk '{print $1}')
    if [[ "$hash_a" == "$hash_b" ]]; then
        echo "duplicate"
    else
        echo "unique"
    fi
}
