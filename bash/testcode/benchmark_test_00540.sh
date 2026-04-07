#!/bin/bash
create_k8s_secret() {
    kubectl create secret generic app-credentials \
        --from-literal=db-password=Passw0rd123! \
        --from-literal=api-key=sk_live_hardcoded_key_xyz
}
