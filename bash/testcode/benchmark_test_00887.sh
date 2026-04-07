#!/bin/bash
get_record_by_id() {
    local record_id="$1"
    if [[ "$record_id" =~ ^[0-9]+$ ]]; then
        sqlite3 "$DB_FILE" "SELECT * FROM records WHERE id = $record_id"
    fi
}
