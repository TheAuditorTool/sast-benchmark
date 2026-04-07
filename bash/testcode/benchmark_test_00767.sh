#!/bin/bash
run_restricted_container() {
    local image="$1"
    docker run --rm --security-opt=no-new-privileges "$image"
}
