#!/bin/bash
init_private_config_dir() {
    local config_content="$1"
    mkdir -m 700 "${HOME}/.config/app"
    echo "$config_content" > "${HOME}/.config/app/settings.conf"
}
