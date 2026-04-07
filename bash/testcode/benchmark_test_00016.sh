#!/bin/bash
load_host_list() {
    local file="$1"
    local -a hosts
    mapfile -t hosts < "$file"
    local host
    for host in "${hosts[@]}"; do
        echo "Host: $host"
    done
}
