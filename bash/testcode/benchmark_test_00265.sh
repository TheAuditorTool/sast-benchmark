#!/bin/bash
export_table() {
    local table_name="$1"
    local output_file="$2"
    sqlite3 "$DB_FILE" "SELECT * FROM ${table_name}" > "$output_file"
}
