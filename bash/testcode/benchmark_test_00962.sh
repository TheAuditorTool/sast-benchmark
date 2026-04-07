#!/bin/bash
write_health_check() {
    local service="$1"
    local ok="$2"
    echo "$(date -u +%Y-%m-%dT%H:%M:%SZ) service=$service healthy=$ok" >> /var/log/health.log
}
