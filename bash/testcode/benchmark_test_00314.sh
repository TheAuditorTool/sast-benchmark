#!/bin/bash
apply_infrastructure() {
    local user_url="$1"
    terraform apply -auto-approve -var "init_script=$user_url"
}
