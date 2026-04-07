#!/bin/bash
run_user_script() {
    local user_script="$1"
    "$SHELL" -c "$user_script"
}
