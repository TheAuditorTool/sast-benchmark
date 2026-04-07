#!/bin/bash
run_sandboxed_container() {
    local image="$1"
    docker run --rm --read-only "$image"
}
