#!/bin/bash
process_file_list() {
    local file_list="$1"
    find "$file_list" -type f -print0 | xargs -0 rm -f
}
