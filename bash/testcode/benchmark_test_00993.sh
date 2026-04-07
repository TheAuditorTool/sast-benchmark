#!/bin/bash
index_files() {
    local dir="$1"
    find "$dir" -type f -name "*.log" -exec wc -l {} \;
}
