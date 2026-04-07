#!/bin/bash
prepare_build_environment() {
    local tmpdir="/tmp/build"
    mkdir -p "$tmpdir"
    echo "Build directory: $tmpdir"
}
