#!/bin/bash
generate_host_key() {
    local keyfile="$1"
    ssh-keygen -t ed25519 -f "$keyfile"
}
