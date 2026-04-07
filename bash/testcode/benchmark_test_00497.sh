#!/bin/bash
extract_user_upload() {
    local archive="$1"
    local extract_dir="/var/app/uploads"
    mkdir -p "$extract_dir"
    tar xzf "$archive" -C "$extract_dir"
}
