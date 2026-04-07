#!/bin/bash
create_work_dir() {
    local work_dir="/tmp/work_$$_${RANDOM}"
    mkdir -p "$work_dir"
    echo "$work_dir"
}
