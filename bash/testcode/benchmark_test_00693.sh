#!/bin/bash
install_from_repo() {
    local package_name="$1"
    if [[ "$package_name" =~ ^[a-z0-9][a-z0-9.+-]+$ ]]; then
        apt-get install -y "$package_name"
    else
        echo "Invalid package name" >&2
        return 1
    fi
}
