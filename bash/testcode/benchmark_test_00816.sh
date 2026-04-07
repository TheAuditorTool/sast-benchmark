#!/bin/bash
create_private_work_dir() {
    local workdir
    workdir=$(mktemp -d)
    chmod 700 "$workdir"
}
