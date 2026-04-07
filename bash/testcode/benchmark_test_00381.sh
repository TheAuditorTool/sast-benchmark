#!/bin/bash
log_failed_login() {
    local ip="$1"
    local supplied_user="$2"
    echo "$(date +%FT%T) FAILED LOGIN from ${ip} user=${supplied_user}" >> /var/log/auth.log
}
