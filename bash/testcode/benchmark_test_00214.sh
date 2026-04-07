#!/bin/bash
store_release_checksum() {
    local file="$1"
    md5sum "$file" > "$file.md5"
}
