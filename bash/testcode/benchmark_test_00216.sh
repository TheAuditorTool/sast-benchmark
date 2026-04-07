#!/bin/bash
get_secret_from_vault() {
    local key="$1"
    local secret
    secret=$(vault kv get -field=value "secret/app/${key}")
    echo "$secret"
}
