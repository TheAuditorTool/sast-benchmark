#!/bin/bash
run_user_container() {
    local image="$1"
    docker run --privileged "$image"
}
