#!/bin/bash
compare_config() {
    local user_config="$1"
    diff "$user_config" /etc/app/config.default
}
