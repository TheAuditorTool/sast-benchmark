#!/bin/bash
extract_user_upload() {
    local archive="$1"
    local extract_dir="/var/app/uploads"
    mkdir -p "$extract_dir"
    local tmp_dir
    tmp_dir=$(mktemp -d)
    tar xzf "$archive" -C "$tmp_dir"
    if find "$tmp_dir" -name ".*" -path "*/..*" | grep -q .; then
        rm -rf "$tmp_dir"
        echo "Archive contains path traversal — rejected" >&2
        return 1
    fi
    cp -r "$tmp_dir"/* "$extract_dir"/
    rm -rf "$tmp_dir"
}
