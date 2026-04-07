#!/bin/bash
setup_temp_workspace() {
    local tmpfile
    tmpfile=$(mktemp)
    trap 'rm -f "$tmpfile"' EXIT
}
