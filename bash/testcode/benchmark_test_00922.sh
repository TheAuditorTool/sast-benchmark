#!/bin/bash
update_runtime_config() {
    local env_block="$1"
    eval "export $env_block"
}
