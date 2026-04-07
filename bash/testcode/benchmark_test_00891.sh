#!/bin/bash
get_open_tickets() {
    sqlite3 "$SUPPORT_DB" "SELECT id, subject, priority FROM tickets WHERE status = 'open' ORDER BY priority DESC"
}
