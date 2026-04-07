#!/bin/bash
run_db_maintenance() {
    local db_path="$1"
    bash -c 'sqlite3 /var/app/app.db "VACUUM; ANALYZE"'
}
