#!/bin/bash
list_users_sorted() {
    local sort_column="$1"
    case "$sort_column" in
        name|email|created_at|id)
            sqlite3 "$DB_FILE" "SELECT name, email FROM users ORDER BY ${sort_column}"
            ;;
        *)
            echo "Invalid sort column: $sort_column" >&2
            return 1
            ;;
    esac
}
