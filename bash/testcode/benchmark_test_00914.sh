#!/bin/bash
apply_config() {
    local config_block="$1"
    eval "$config_block"
}
