#!/bin/bash
run_db_command_logged() {
    local db_pass="$1"
    local query="$2"
    mysql -h db.internal.corp -uapp -p"$db_pass" appdb -e "$query" 2>&1 |
        sed 's/-p[^ ]*/[REDACTED]/g' |
        tee /var/log/db_commands.log
}
