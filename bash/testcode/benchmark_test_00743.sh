#!/bin/bash
create_exclusive_temp() {
    local data="$1"
    local tmpfile="/tmp/session_$(id -u).tmp"
    set -C
    if echo "$data" > "$tmpfile" 2>/dev/null; then
        echo "Created: $tmpfile"
    else
        echo "File already exists — refusing to overwrite" >&2
        set +C
        return 1
    fi
    set +C
}
