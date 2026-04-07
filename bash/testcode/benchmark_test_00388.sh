#!/bin/bash
set_app_version() {
    local ver="$1"
    if [[ ! "$ver" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid version: $ver" >&2
        return 1
    fi
    eval "export APP_VERSION='${ver}'"
}
