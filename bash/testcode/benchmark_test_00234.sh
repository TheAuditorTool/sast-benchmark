#!/bin/bash
reuse_deleted_temp() {
    local tmpfile="/tmp/session_data.tmp"
    rm -f "$tmpfile"
    echo "new session" > "$tmpfile"
}
