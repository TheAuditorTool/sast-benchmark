#!/bin/bash
check_api_health() {
    local http_code
    http_code=$(curl -s -o /dev/null -w "%{http_code}" "https://api.example.com/health")
    echo "API health check: HTTP ${http_code}"
}
