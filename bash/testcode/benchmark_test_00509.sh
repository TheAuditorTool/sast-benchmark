#!/bin/bash
store_temp_key() {
    local key_material="$1"
    local tmpfile
    tmpfile=$(mktemp)
    trap 'shred -u "$tmpfile"' EXIT
    echo "$key_material" > "$tmpfile"
}
