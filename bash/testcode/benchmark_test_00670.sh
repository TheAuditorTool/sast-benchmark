#!/bin/bash
provision_wireguard_key() {
    local KEYPAIR
    KEYPAIR=$(wg genkey)
    echo "$KEYPAIR"
}
