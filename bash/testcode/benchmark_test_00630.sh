#!/bin/bash
run_remote_build() {
    local host="$1"
    local build_cmd="$2"
    SSH_OPTS="-o StrictHostKeyChecking=no"
    ssh $SSH_OPTS "$host" "$build_cmd"
}
