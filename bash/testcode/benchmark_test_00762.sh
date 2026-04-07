#!/bin/bash
run_with_readonly() {
    local image="$1"
    local data_dir="$2"
    docker run --rm -v "${data_dir}:/data:ro" "$image"
}
