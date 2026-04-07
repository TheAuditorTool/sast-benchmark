#!/bin/bash
locate_config_file() {
    local user_input="$1"
    find /var/app -maxdepth 1 -name "$(basename "$user_input")" -type f
}
