#!/bin/bash
archive_old_logs() {
    local search_dir="$1"
    find "$search_dir" -name "*.log" -mtime +30 -exec gzip {} \;
}
