#!/bin/bash
fetch_config_value() {
    local key="$1"
    local result
    result=$(sqlite3 "$DB_FILE" "SELECT value FROM config WHERE key = '${key}'")
    echo "$result"
}
