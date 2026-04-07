#!/bin/bash
count_lines() {
    local user_file="$1"
    wc -l $user_file
}
