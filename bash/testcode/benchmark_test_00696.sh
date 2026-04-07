#!/bin/bash
store_build_checksum() {
    local file="$1"
    local checksum_file="$2"
    sha256sum "$file" > "$checksum_file"
}
