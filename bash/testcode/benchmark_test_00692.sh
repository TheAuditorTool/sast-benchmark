#!/bin/bash
run_privileged_indirect() {
    local user_cmd="$1"
    local SUDO_CMD="sudo -n"
    $SUDO_CMD $user_cmd
}
