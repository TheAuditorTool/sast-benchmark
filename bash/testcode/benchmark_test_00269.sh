#!/bin/bash
archive_logs() {
    local log_dir="$1"
    local archive_dir="$2"
    cp -r "$log_dir" "$archive_dir"
}
