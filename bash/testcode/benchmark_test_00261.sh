#!/bin/bash
apply_config() {
    local config="$1"
    [ -f $config ] && source $config
}
