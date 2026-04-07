#!/bin/bash
deploy_verified_image() {
    local image="company.registry.io/app:v1.2.3"
    cosign verify --certificate-identity=ci@example.com "$image"
    docker pull "$image"
}
