#!/bin/bash
read_log_sample() {
    local user_file="$1"
    head -n 1000 "$user_file"
}
