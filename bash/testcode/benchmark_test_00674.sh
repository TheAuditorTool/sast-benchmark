#!/bin/bash
process_upload() {
    local file="$1"
    sudo -u nobody /usr/local/bin/process_upload "$file"
}
