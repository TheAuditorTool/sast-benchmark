#!/bin/bash
get_secret_from_ssm() {
    local param_name="$1"
    local secret
    secret=$(aws ssm get-parameter --name "$param_name" \
        --with-decryption --query "Parameter.Value" --output text)
    echo "$secret"
}
