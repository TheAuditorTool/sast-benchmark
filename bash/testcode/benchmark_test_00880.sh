#!/bin/bash
query_audit_log() {
    local actor="$1"
    local action="$2"
    sqlite3 "$AUDIT_DB" "SELECT event_time, details FROM audit WHERE actor = '${actor}' AND action = '${action}'"
}
