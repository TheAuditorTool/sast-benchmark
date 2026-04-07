#!/bin/bash
read_config_line() {
    local config_file="$1"
    local key="$2"
    grep "^${key}=" "$config_file" | cut -d= -f2-
}
