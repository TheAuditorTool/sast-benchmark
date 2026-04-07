#!/bin/bash
stage_upload() {
    local filename="$1"
    local staging_path="/tmp/upload_$$"
    cp "$filename" "$staging_path"
    echo "$staging_path"
}
