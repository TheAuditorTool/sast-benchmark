#!/bin/bash
log_status_change() {
    local new_status="$1"
    case "$new_status" in
        active|inactive|suspended|deleted)
            echo "$(date +%F) Status changed to: ${new_status}" >> /var/log/status.log
            ;;
        *)
            echo "Invalid status value" >&2
            return 1
            ;;
    esac
}
