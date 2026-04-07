#!/bin/bash
run_cleanup_on_files() {
    local cleanup_cmd="$1"
    local search_dir="$2"
    find "$search_dir" -name "*.tmp" -exec $cleanup_cmd {} \;
}
