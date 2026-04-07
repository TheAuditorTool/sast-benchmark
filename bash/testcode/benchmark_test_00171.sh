#!/bin/bash
save_upload() {
    local raw_filename="$1"
    local content="$2"
    local filename
    filename=$(echo "$raw_filename" | tr -dc 'A-Za-z0-9._-')
    echo "$content" > "/tmp/staging/${filename}"
}
