#!/bin/bash
extract_upload() {
    local archive="$1"
    local dest="$2"
    tar xzf "$archive" -C "$dest"
}
