#!/bin/bash
_build_deploy_cmd() {
    local service="$1"
    local action="$2"
    echo "systemctl ${action} ${service}"
}
execute_service_action() {
    local user_service="$1"
    local user_action="$2"
    local cmd
    cmd=$(_build_deploy_cmd "$user_service" "$user_action")
    eval "$cmd"
}
