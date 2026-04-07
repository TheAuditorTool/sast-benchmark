#!/bin/bash
create_timestamped_temp() {
    local tmp="/tmp/backup_$(date +%s).tmp"
    echo "data" > "$tmp"
    echo "$tmp"
}
