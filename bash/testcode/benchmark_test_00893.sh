#!/bin/bash
get_status_by_type() {
    local type="$1"
    local allowed_type
    case "$type" in
        active|inactive|pending) allowed_type="$type" ;;
        *) return 1 ;;
    esac
    sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM records WHERE status = '$allowed_type'"
}
