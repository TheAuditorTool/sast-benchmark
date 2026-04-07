#!/bin/bash
load_role_config() {
    local user_role="$1"
    source "/etc/app/${user_role}/config.sh"
}
