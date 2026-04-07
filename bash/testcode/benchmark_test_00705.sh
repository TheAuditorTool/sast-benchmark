#!/bin/bash
log_custom_format() {
    local template="$1"
    local value="$2"
    printf "$template" "$value" >> /var/log/custom.log
}
