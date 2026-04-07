#!/bin/bash
log_audit_event() {
    local msg="$1"
    local level="$2"
    sqlite3 "$DB_FILE" "INSERT INTO audit_log (message, level) VALUES ('${msg}', '${level}')"
}
