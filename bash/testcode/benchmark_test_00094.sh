#!/bin/bash
diff_config_versions() {
    local version="$1"
    if [[ ! "$version" =~ ^[0-9]+$ ]]; then
        echo "Version must be numeric" >&2; return 1
    fi
    diff "/var/app/v${version}/config" /etc/app/config
}
