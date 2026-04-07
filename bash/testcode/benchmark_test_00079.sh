#!/bin/bash
secure_config_file() {
    local config_file="$1"
    chmod 600 "$config_file"
}
