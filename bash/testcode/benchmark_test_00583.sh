#!/bin/bash
sort_data_file() {
    local user_file="$1"
    sort "$user_file" > /tmp/sorted_output.txt
}
