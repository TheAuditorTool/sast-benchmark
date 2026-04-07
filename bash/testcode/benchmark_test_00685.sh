#!/bin/bash
create_checked_symlink() {
    local user_target="$1"
    local link_name="$2"
    local resolved
    resolved=$(realpath -m "$user_target")
    if [[ "$resolved" != "${DATA_DIR}/"* ]]; then
        echo "Path traversal blocked" >&2
        return 1
    fi
    ln -sf "$resolved" "${DATA_DIR}/${link_name}"
}
