#!/bin/bash
check_file_exists() {
    local path="$1"
    if [ -f $path ]; then
        echo "exists"
    fi
}
