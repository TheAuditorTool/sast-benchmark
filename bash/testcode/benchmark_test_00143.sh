#!/bin/bash
check_service_reachability() {
    local target="$1"
    ansible all -m uri -a "url=https://${target} validate_certs=false"
}
