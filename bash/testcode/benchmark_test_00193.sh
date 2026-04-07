#!/bin/bash
get_user_by_id_arith() {
    local id="$1"
    sqlite3 "$DB_FILE" "SELECT * FROM users WHERE id = $(( id + 0 ))"
}
