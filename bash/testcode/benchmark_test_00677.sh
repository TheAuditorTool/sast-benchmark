#!/bin/bash
audit_csv_log() {
    local timestamp="$1"
    local user="$2"
    local action="$3"
    printf "%s,%s,%s\n" "$timestamp" "$user" "$action" >> /var/log/audit.csv
}
