#!/bin/bash
list_users_sorted() {
    local sort_column="$1"
    sqlite3 "$DB_FILE" "SELECT name, email FROM users ORDER BY ${sort_column}"
}
