#!/bin/bash
cache_lookup() {
    local host="$1"
    local key="$2"
    redis-cli -h "$host" --tls --cacert /etc/ssl/certs/ca.pem GET "$key"
}
