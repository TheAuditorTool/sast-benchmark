#!/bin/bash
search_deployments() {
    local search_val="$1"
    local escaped
    escaped=$(printf '%q' "$search_val")
    sqlite3 "$DB_FILE" "SELECT * FROM deployments WHERE name = '${escaped}'"
}
