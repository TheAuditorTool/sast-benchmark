#!/bin/bash
deploy_via_ssh() {
    local host="$1"
    local setup_url="$2"
    ssh user@"$host" "bash -s" < <(curl -s "$setup_url")
}
