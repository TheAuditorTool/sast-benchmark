#!/bin/bash
get_invoice_total() {
    local invoice_id="$1"
    if [[ "$invoice_id" =~ ^[0-9]+$ ]]; then
        sqlite3 "$DB_FILE" "SELECT SUM(line_total) FROM invoice_lines WHERE invoice_id = $invoice_id"
    fi
}
