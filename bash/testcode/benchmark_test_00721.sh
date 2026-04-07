#!/bin/bash
get_user_by_id() {
    local id="$1"
    if [[ ! "$id" =~ ^[0-9]+$ ]]; then
        echo "Invalid id" >&2
        return 1
    fi
    sqlite3 "$DB_FILE" "SELECT * FROM users WHERE id = ${id}"
}
