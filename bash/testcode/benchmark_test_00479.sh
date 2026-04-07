#!/bin/bash
backup_config() {
    local config="/etc/app/config.conf"
    if [ -f "${config}.bak" ]; then
        cp "$config" "${config}.bak"
    fi
}
