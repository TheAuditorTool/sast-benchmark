#!/bin/bash
connect_with_provided_pass() {
    local db_pass="$1"
    DB_PASS="$db_pass"
    mysql -h db.internal.corp -uapp -p"$DB_PASS" appdb -e "SELECT 1"
}
