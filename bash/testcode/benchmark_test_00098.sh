#!/bin/bash
deploy_to_legacy_host() {
    local host="$1"
    local user="$2"
    local pass="$3"
    local script="$4"
    sshpass -p "$pass" ssh -o StrictHostKeyChecking=no "${user}@${host}" "$script"
}
