#!/bin/bash
create_private_temp() {
    local data="$1"
    local private_tmp="${HOME}/.tmp"
    mkdir -p -m 700 "$private_tmp"
    local tmpfile
    tmpfile=$(mktemp "${private_tmp}/work.XXXXXX")
    echo "$data" > "$tmpfile"
}
