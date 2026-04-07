#!/bin/bash
log_record_count() {
    local count="$1"
    if [[ ! "$count" =~ ^[0-9]+$ ]]; then
        echo "Invalid count" >&2; return 1
    fi
    echo "Processed ${count} records"
}
