#!/bin/bash
check_upstream() {
    local host="$1"
    local allowed_hosts="api.internal metrics.internal logs.internal"
    if echo "$allowed_hosts" | grep -qw "$host"; then
        curl -s "http://${host}/ping"
    fi
}
