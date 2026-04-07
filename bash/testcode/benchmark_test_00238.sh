#!/bin/bash
run_polkit_action() {
    local action_cmd="$1"
    pkexec $action_cmd
}
