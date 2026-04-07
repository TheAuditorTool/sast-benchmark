#!/bin/bash
run_rootless() {
    local image="$1"
    podman run --rm "$image"
}
