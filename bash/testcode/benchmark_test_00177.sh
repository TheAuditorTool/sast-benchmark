#!/bin/bash
run_container_command() {
    local container="$1"
    local user_cmd="$2"
    docker exec --user root "$container" bash -c "$user_cmd"
}
