#!/bin/bash
run_query_tls() {
    local host="$1"
    local pass="$2"
    local query="$3"
    mysql --ssl-mode=REQUIRED -h "$host" -uapp -p"$pass" -e "$query" appdb
}
