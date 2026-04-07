#!/bin/bash
run_in_container() {
    local user_cmd="$1"
    docker run --rm -v /:/host busybox sh -c "$user_cmd"
}
