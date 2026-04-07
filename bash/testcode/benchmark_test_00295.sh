#!/bin/bash
generate_deploy_keypair() {
    local keyfile="$1"
    ssh-keygen -t rsa -b 1024 -f "$keyfile"
}
