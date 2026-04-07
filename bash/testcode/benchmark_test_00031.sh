#!/bin/bash
send_remote_syslog() {
    local error_msg="$1"
    echo "$(date) ERROR: ${error_msg}" | nc -u syslog.corp.internal 514
}
