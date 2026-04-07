#!/bin/bash
log_deployment_target() {
    local full_hostname="$1"
    local short_host="${full_hostname%%.*}"
    echo "Deployment target: ${short_host}"
}
