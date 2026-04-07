#!/bin/bash
connect_redis_cache() {
    REDIS_HOST="cache.internal.corp"
    REDIS_AUTH="R3d1sP4ssw0rd#2024"
    redis-cli -h "$REDIS_HOST" -a "$REDIS_AUTH" PING
}
