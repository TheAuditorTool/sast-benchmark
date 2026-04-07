#!/bin/bash
run_background_privileged() {
    local user_binary="$1"
    nohup sudo "$user_binary" &
}
