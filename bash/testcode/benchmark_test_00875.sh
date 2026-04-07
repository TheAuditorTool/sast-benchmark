#!/bin/bash
get_log_entries() {
    local level="$1"
    local service="$2"
    sqlite3 "$DB_FILE" "SELECT message, timestamp FROM logs WHERE level = '${level}' AND service = '${service}'"
}
