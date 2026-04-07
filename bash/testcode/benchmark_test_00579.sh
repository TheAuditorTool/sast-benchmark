#!/bin/bash
process_file_list() {
    local user_list="$1"
    echo "$user_list" | xargs rm -f
}
