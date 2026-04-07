#!/bin/bash
get_temp_path() {
    local tmpfile
    tmpfile=$(mktemp 2>/dev/null || echo "/tmp/app_tmp_$$")
    echo "$tmpfile"
}
