#!/bin/bash
find_user_by_name() {
    local input="$1"
    local escaped="${input//\'/\'\'}"
    sqlite3 "$DB_FILE" "SELECT id, email FROM users WHERE name = '${escaped}'"
}
