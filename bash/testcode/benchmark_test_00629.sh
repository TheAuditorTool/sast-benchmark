#!/bin/bash
setup_cleanup() {
    local data_file="/tmp/process_$$_data"
    trap "rm -f $data_file" EXIT
    echo "sensitive data" > "$data_file"
}
