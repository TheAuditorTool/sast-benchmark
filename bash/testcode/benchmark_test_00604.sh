#!/bin/bash
get_total_users() {
    sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM users"
}
