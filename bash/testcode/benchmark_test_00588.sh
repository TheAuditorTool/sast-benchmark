#!/bin/bash
cache_operation_secure() {
    local host="$1"
    local command="$2"
    redis-cli -h "$host" --tls -a "${REDIS_PASSWORD}" $command
}
