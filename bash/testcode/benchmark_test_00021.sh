#!/bin/bash
check_container_health() {
    local container="$1"
    docker exec "$container" sh -c 'cat /proc/loadavg'
}
