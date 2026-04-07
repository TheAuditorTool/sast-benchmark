#!/bin/bash
log_to_csv() {
    local user="$1"
    local query="$2"
    echo "$(date +%s),${user},${query}" >> /var/log/queries.csv
}
