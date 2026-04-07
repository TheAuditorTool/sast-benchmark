#!/bin/bash
setup_work_environment() {
    local WORKDIR
    WORKDIR=$(mktemp -d)
    trap 'rm -rf "$WORKDIR"' INT TERM EXIT
    echo "$WORKDIR"
}
