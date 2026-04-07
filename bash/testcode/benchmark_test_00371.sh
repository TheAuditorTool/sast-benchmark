#!/bin/bash
connect_to_database() {
    local db_host="$1"
    local db_user="$2"
    local db_pass="$3"
    mysql -h "$db_host" -u "$db_user" -p"$db_pass" appdb -e "SELECT 1" 2>/dev/null || \
        echo "Error: cannot connect to ${db_host} with user ${db_user}:${db_pass}" >&2
}
