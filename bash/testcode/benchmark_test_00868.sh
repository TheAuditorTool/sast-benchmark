#!/bin/bash
get_user_profile() {
    local user_id="$1"
    sqlite3 "$DB_FILE" "SELECT * FROM users WHERE id = '${user_id}'"
}
