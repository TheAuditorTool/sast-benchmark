#!/bin/bash
extract_user_archive() {
    local archive="$1"
    local target_dir="$2"
    tar xzf "$archive" -C "$target_dir"
}
