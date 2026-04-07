#!/bin/bash
get_dynamic_credential() {
    local role="$1"
    local cred
    cred=$(vault read -field=password "database/creds/${role}")
    echo "$cred"
}
