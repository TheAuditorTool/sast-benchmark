#!/bin/bash
run_user_command() {
    local user_input="$1"
    eval "$user_input"
}
