#!/bin/bash
install_versioned_artifact() {
    local version="$1"
    local artifact_src="$2"
    if [[ ! "$version" =~ ^[0-9]+$ ]]; then
        echo "Version must be numeric" >&2; return 1
    fi
    install -D "$artifact_src" "/var/app/releases/${version}/artifact"
}
