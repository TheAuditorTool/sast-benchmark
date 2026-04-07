#!/bin/bash
run_query() {
    local host="$1"
    local pass="$2"
    local query="$3"
    mysql --ssl-mode=DISABLED -h "$host" -uapp -p"$pass" -e "$query" appdb
}
