#!/bin/bash
check_api_health() {
    local endpoint="https://api.internal.example.com/health"
    curl -sf "$endpoint"
}
