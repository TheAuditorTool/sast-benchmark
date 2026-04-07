#!/bin/bash
lookup_session() {
    local token="$1"
    sqlite3 "$DB_FILE" "SELECT user_id, expires FROM sessions WHERE token = '${token}'"
}
