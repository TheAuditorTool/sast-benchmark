#!/bin/bash
create_bounded_scratch() {
    local work_dir
    work_dir=$(mktemp -d)
    sudo mount -t tmpfs -o size=100M tmpfs "$work_dir"
    echo "$work_dir"
}
