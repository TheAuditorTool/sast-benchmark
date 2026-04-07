#!/bin/bash
create_work_directory() {
    local work_dir
    work_dir=$(mktemp -d)
    echo "$work_dir"
}
