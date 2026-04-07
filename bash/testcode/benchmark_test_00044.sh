#!/bin/bash
log_client_connection() {
    local ip="$1"
    sanitized_ip=$(printf '%s' "$ip" | tr -dc '0-9.')
    logger -p auth.info -- "Connection from ${sanitized_ip}"
}
