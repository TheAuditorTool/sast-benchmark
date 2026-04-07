#!/bin/bash
query_database_secure() {
    local host="$1"
    local query="$2"
    mysql -h "$host" --ssl-mode=REQUIRED -u app -e "$query" appdb
}
