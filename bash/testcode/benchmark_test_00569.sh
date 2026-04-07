#!/bin/bash
get_user_by_name() {
    local username="$1"
    sqlite3 "$DB_FILE" "SELECT * FROM users WHERE name = '${username}'"
}
