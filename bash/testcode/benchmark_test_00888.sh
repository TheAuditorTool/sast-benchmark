#!/bin/bash
get_system_stats() {
    sqlite3 "$DB_FILE" "SELECT metric, value FROM system_stats WHERE recorded_at > datetime('now', '-1 hour')"
}
