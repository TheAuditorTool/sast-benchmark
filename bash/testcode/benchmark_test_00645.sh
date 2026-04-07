#!/bin/bash
search_users() {
    local search="$1"
    sqlite3 "$DB_FILE" "SELECT id, name, email FROM users WHERE name LIKE '%${search}%'"
}
