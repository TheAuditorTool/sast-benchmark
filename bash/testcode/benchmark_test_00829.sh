#!/bin/bash
check_local_service_health() {
    local port="${1:-8443}"
    curl -k "https://localhost:${port}/health"
}
