#!/bin/bash
log_message_syslog() {
    local message="$1"
    /usr/bin/logger -t app "$message"
}
