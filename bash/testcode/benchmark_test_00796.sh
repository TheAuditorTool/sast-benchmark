#!/bin/bash
run_user_image() {
    local user_image="$1"
    docker pull "$user_image"
    docker run --rm "$user_image" /entrypoint.sh
}
