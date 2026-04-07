#!/bin/bash
deploy_via_ssh_key() {
    local host="$1"
    ssh -i /etc/deploy/keys/id_ed25519 -o IdentitiesOnly=yes "deploy@${host}" \
        'systemctl restart app'
}
