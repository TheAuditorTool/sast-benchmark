#!/bin/bash
run_db_migration() {
    local db_pass="$1"
    set -x
    mysql -h db.internal.corp -uapp -p"$db_pass" appdb -e "source /var/app/migration.sql"
    set +x
}
