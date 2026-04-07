#!/bin/bash
backup_if_owned() {
    local src="$1"
    local dest="$2"
    if [[ "$(stat -c '%U' "$src")" == "$(whoami)" ]]; then
        cp "$src" "$dest"
    fi
}
