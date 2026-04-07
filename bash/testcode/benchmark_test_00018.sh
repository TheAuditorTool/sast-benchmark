#!/bin/bash
use_temporary_credentials() {
    local CREDS_FILE
    CREDS_FILE=$(mktemp)
    trap 'rm -f "$CREDS_FILE"' EXIT
    vault read -format=json "auth/aws/login" > "$CREDS_FILE"
    TOKEN=$(jq -r '.auth.client_token' "$CREDS_FILE")
    export TOKEN
}
