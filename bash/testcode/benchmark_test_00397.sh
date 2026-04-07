#!/bin/bash
read_user_file() {
    local filename="$1"
    local safe_name
    safe_name=$(basename "$filename")
    cat "${DATA_DIR}/${safe_name}"
}
