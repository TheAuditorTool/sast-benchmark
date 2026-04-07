#!/bin/bash
create_install_dir() {
    local work_dir="/tmp/work_$$"
    install -d -m 700 "$work_dir"
    echo "$work_dir"
}
