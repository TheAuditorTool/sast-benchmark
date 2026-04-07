#!/bin/bash
parse_env_file() {
    local file="$1"
    local key value
    while IFS='=' read -r key value; do
        export "$key=$value"
    done < "$file"
}
