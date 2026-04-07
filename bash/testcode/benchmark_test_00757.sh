#!/bin/bash
get_schema_version() {
    sqlite3 "$DB_FILE" "SELECT version FROM schema_meta WHERE id = 1"
}
