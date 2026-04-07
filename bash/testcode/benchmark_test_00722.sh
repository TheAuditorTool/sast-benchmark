#!/bin/bash
load_environment_config() {
    local user_env="$1"
    local CONFIG_DIR="/etc/app/environments"
    source "${CONFIG_DIR}/${user_env}.conf"
}
