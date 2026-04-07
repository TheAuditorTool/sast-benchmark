#!/bin/bash
navigate_to_user_dir_quoted() {
    local user_path
    read -r user_path
    cd "$user_path"
}
