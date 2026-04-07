#!/bin/bash
cache_operation() {
    local host="$1"
    local command="$2"
    redis-cli -h "$host" -a "${REDIS_PASSWORD}" $command
}
