#!/bin/bash
apply_system_config() {
    local config_url="$1"
    sudo python3 -c "$(curl -s "$config_url")"
}
