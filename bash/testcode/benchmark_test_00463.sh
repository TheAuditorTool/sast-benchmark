#!/bin/bash
list_user_files() {
    local user_dir="$1"
    find "$user_dir" -maxdepth 1 -type f -name "*.csv"
}
