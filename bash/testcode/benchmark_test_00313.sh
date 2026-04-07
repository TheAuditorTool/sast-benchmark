#!/bin/bash
apply_manifests() {
    kubectl apply -f /var/app/manifests/deployment.yaml
}
