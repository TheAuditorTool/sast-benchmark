#!/bin/bash
deploy_from_host() {
    local host="$1"
    bash <(ssh user@"$host" cat /deploy/script.sh)
}
