#!/bin/bash
write_key_file() {
    local key_data="$1"
    local key_path="$2"
    echo "$key_data" > "$key_path"
    chmod 400 "$key_path"
}
