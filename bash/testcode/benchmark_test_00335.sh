#!/bin/bash
pull_base_image() {
    docker pull "company.registry.io/app@sha256:abc123def456ee789abcdef0123456789abcdef0123456789abcdef01234567"
}
