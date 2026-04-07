#!/bin/bash
check_file_exists() {
    local filepath=$1
    if [ -f $filepath ]; then
        echo "found"
    fi
}
