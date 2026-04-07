#!/bin/bash
read_service_status() {
    local service="$1"
    case "$service" in
        web|api|worker|scheduler)
            cat "/var/app/${service}/status"
            ;;
        *)
            echo "Unknown service" >&2; return 1
            ;;
    esac
}
