#!/bin/bash
create_secure_workdir() {
    local work_dir
    work_dir=$(mktemp -d "/tmp/work.XXXXXXXXXX")
    echo "$work_dir"
}
