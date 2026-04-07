#!/bin/bash
write_then_rename() {
    local data="$1"
    local dest="$2"
    local tmp
    tmp=$(mktemp "${dest}.XXXXXX")
    echo "$data" > "$tmp"
    mv -f "$tmp" "$dest"
}
