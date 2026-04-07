#!/bin/bash
log_connection_source() {
    local ip="$1"
    clean_ip="${ip//[^0-9.]/}"
    logger -e "Connection from ${clean_ip}"
}
