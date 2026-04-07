#!/bin/bash
deploy_config() {
    local new_config="$1"
    cp "$new_config" /etc/app/config.conf
}
