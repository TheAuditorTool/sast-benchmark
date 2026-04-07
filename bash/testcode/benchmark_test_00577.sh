#!/bin/bash
get_active_deployments() {
    sqlite3 "$DB_FILE" "SELECT * FROM deployments WHERE status = 'active' ORDER BY created_at DESC LIMIT 10"
}
