#!/bin/bash
ssh_first_connect() {
    local host="$1"
    local cmd="$2"
    ssh -o StrictHostKeyChecking=accept-new "$host" "$cmd"
}
