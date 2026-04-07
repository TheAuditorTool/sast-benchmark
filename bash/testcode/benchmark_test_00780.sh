#!/bin/bash
query_database() {
    local host="$1"
    local query="$2"
    mysql -h "$host" -u app -pSecret123 -e "$query" appdb
}
