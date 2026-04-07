#!/bin/bash
install_plugin() {
    local plugin_url="$1"
    local tmpfile
    tmpfile=$(mktemp)
    wget -q -O "$tmpfile" "$plugin_url"
    source "$tmpfile"
}
