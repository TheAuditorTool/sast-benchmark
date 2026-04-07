#!/bin/bash
pull_versioned_image() {
    local version_tag="$1"
    docker pull "registry.company.internal/app:${version_tag}"
}
