#!/bin/bash
call_internal_service() {
    local service_name="$1"
    declare -A SERVICE_URLS=(
        [auth]="https://auth.internal.corp/health"
        [metrics]="https://metrics.internal.corp/health"
        [cache]="https://cache.internal.corp/health"
    )
    curl -s "${SERVICE_URLS[$service_name]}"
}
