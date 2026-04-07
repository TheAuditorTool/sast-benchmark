#!/bin/bash
append_access_log() {
    local level="$1"
    local msg="$2"
    echo "[${level}] ${msg}" >> /var/log/app.log
}
