#!/bin/bash
run_container_host_mount() {
    local image="$1"
    docker run -v /:/host "$image"
}
