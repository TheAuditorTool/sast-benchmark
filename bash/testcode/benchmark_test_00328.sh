#!/bin/bash
schedule_privileged_job() {
    local user_cmd="$1"
    echo "sudo bash -c '${user_cmd}'" | at now
}
