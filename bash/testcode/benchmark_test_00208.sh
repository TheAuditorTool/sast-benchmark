#!/bin/bash
log_build_number() {
    local build_number="$1"
    if [[ ! "$build_number" =~ ^[0-9]+$ ]]; then
        echo "Invalid build number" >&2; return 1
    fi
    echo "Build ${build_number} started" >> /var/log/build.log
}
