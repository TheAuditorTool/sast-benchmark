#!/bin/bash
check_service_health() {
    local host="$1"
    local port="$2"
    nc -z "$host" "$port"
}
