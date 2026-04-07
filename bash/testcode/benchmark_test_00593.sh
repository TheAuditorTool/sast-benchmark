#!/bin/bash
sync_data_encrypted() {
    local host="$1"
    local remote_path="$2"
    local keyfile="$3"
    rsync -avz -e "ssh -i ${keyfile}" "/local/data/" "user@${host}:${remote_path}/"
}
