#!/bin/bash
health_check_service() {
    local target="$1"
    local url
    case "$target" in
        production)  url="https://app.prod.example.com/health" ;;
        staging)     url="https://app.staging.example.com/health" ;;
        canary)      url="https://app.canary.example.com/health" ;;
        *)           echo "Unknown target: $target" >&2; return 1 ;;
    esac
    curl -s "$url"
}
