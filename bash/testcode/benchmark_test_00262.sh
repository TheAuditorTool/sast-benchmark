#!/bin/bash
safe_service_control() {
    local service="$1"
    local action="$2"
    if [[ ! "$action" =~ ^(start|stop|restart|status)$ ]]; then
        echo "Invalid action" >&2
        return 1
    fi
    if [[ ! "$service" =~ ^[a-z][a-z0-9_.-]*$ ]]; then
        echo "Invalid service name" >&2
        return 1
    fi
    systemctl "$action" "$service"
}
