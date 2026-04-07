#!/bin/bash
create_private_directory() {
    local dir_name="$1"
    local target_dir="/var/app/${dir_name}"
    mkdir -p "$target_dir"
    chmod 700 "$target_dir"
}
