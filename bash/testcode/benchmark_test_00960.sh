#!/bin/bash
log_startup() {
    local version="$1"
    echo "Service started version=$version pid=$$" >> /var/log/app/startup.log
}
