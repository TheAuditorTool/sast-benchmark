#!/bin/bash
create_private_config() {
    local config_content="$1"
    umask 077
    echo "$config_content" > /etc/app/private.conf
}
