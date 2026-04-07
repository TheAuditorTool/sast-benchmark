#!/bin/bash
fetch_monthly_report() {
    local month="$1"
    if [[ "$month" =~ ^[0-9]{4}-[0-9]{2}$ ]]; then
        sqlite3 "$DB_FILE" "SELECT SUM(amount) FROM transactions WHERE strftime('%Y-%m', date) = '$month'"
    fi
}
