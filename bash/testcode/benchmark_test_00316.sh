#!/bin/bash
deploy_services() {
    local -a services=("$@")
    local svc
    for svc in "${services[@]}"; do
        systemctl restart "$svc"
    done
}
