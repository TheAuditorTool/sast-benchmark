#!/bin/bash
set_config_value() {
    local user_key="$1"
    local value="$2"
    eval "config[${user_key}]='${value}'"
}
