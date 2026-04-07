#!/bin/bash
log_script_name() {
    local name
    name=$(basename "$0")
    echo "Script: $name"
}
