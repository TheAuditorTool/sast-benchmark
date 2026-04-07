#!/bin/bash
log_build_metadata() {
    local build_id="$1"
    set -x
    echo "Starting build pipeline for build ID: ${build_id}"
    set +x
}
