#!/bin/bash
list_invoices() {
    local status="$1"
    sqlite3 "$DB_FILE" "SELECT id, amount FROM invoices WHERE status = '${status}' ORDER BY created_at"
}
