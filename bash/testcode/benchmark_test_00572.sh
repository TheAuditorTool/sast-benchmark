#!/bin/bash
connect_to_production() {
    local user="$1"
    local host="$2"
    ssh -o StrictHostKeyChecking=yes -o UserKnownHostsFile=/etc/ssh/known_hosts "${user}@${host}"
}
