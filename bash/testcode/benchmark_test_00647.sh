#!/bin/bash
read_file_checked() {
    local user_path="$1"
    local real_path
    real_path=$(realpath -m "${DATA_DIR}/${user_path}")
    if [[ "$real_path" != "${DATA_DIR}/"* ]]; then
        echo "Path traversal attempt blocked" >&2
        return 1
    fi
    cat "$real_path"
}
