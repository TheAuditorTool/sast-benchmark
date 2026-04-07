#!/bin/bash
dump_config_files() {
    local user_dir="$1"
    find "$user_dir" -name "*.conf" -exec cat {} \;
}
