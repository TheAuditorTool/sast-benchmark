#!/bin/bash
count_active_users() {
    sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM users WHERE active = 1"
}
