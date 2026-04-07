#!/bin/bash
store_artifact_checksum() {
    local file="$1"
    b2sum "$file" > "$file.blake2"
}
