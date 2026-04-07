#!/bin/bash
serve_user_file() {
    local name="$1"
    local DATA_DIR="/var/app/uploads"
    local resolved
    resolved=$(readlink -f "${DATA_DIR}/${name}")
    if [[ "$resolved" != "${DATA_DIR}/"* ]]; then
        echo "Path traversal blocked: $name" >&2
        return 1
    fi
    cat "$resolved"
}
