#!/bin/bash
manage_device_secure() {
    local host="$1"
    local command="$2"
    ssh "admin@${host}" "$command"
}
