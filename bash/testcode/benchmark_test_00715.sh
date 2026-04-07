#!/bin/bash
connect_to_bastion() {
    local user="$1"
    local host="$2"
    ssh -o StrictHostKeyChecking=yes "${user}@${host}"
}
