#!/bin/bash
count_active_sessions() {
    sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM active_sessions WHERE expires_at > strftime('%s','now')"
}
