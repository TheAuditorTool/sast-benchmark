#!/bin/bash
activate_feature() {
    local feature="$1"
    if [[ ! "$feature" =~ ^[a-z_]+$ ]]; then
        echo "Invalid feature name: $feature" >&2
        return 1
    fi
    source "/etc/app/features/${feature}.sh"
}
