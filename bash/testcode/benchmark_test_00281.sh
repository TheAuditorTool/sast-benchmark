#!/bin/bash
run_remote_command_quoted() {
    local host="$1"
    local cmd="$2"
    ssh "$host" "$cmd"
}
