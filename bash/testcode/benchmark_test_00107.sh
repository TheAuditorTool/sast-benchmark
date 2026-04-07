#!/bin/bash
count_lines_quoted() {
    local user_file="$1"
    wc -l "$user_file"
}
