#!/bin/bash
load_config_by_name() {
    local config_name="$1"
    if [[ ! "$config_name" =~ ^[a-zA-Z0-9_-]+$ ]]; then
        echo "Invalid config name" >&2; return 1
    fi
    cat "/etc/app/configs/${config_name}.json"
}
