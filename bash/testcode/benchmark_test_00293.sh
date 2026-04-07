#!/bin/bash
query_analytics() {
    local host="$1"
    local pass="$2"
    local query="$3"
    clickhouse-client --host "$host" --password "$pass" --query "$query"
}
