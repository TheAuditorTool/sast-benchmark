#!/bin/bash
aggregate_log_entries() {
    local user_notes="$1"
    local log_line="[$(date +%FT%T)] notes=${user_notes}"
    echo "$log_line" >> /var/log/notes.log
}
