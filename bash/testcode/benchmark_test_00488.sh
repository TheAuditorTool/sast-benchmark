#!/bin/bash
apply_config_quoted() {
    local config="$1"
    [[ -f "$config" ]] && source "$config"
}
