#!/bin/bash
read_config_entries() {
    local config_file="$1"
    local key value
    while IFS='=' read -r key value; do
        echo "Config: $key = $value"
    done < <(grep -v '^#' "$config_file")
}
