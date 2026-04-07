#!/bin/bash
list_users_by_column() {
    local col="$1"
    case "$col" in
        id|name|email|created_at)
            sqlite3 "$DB_FILE" "SELECT * FROM users ORDER BY ${col}"
            ;;
        *)
            echo "Invalid column" >&2
            return 1
            ;;
    esac
}
