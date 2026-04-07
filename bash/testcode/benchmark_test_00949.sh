#!/bin/bash
archive_logs() {
    local log_dir="$1"
    local archive_name="$2"
    tar -czf "${archive_name}.tar.gz" "$log_dir"
}
