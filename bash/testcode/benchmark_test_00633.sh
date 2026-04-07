#!/bin/bash
pull_remote_data() {
    local remote_host="$1"
    rsync -avz "user@${remote_host}:/data/" ./sync/
}
