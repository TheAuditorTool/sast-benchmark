#!/bin/bash
run_privileged_command() {
    local user_cmd="$1"
    sudo -u root bash -c "$user_cmd"
}
