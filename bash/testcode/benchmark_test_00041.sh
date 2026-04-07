#!/bin/bash
clone_approved_repo() {
    local name="$1"
    if [[ ! "$name" =~ ^[a-z][a-z0-9-]+$ ]]; then
        echo "Invalid repo name: $name" >&2
        return 1
    fi
    git clone "git@git.corp.internal:ops/${name}.git" "/tmp/${name}"
}
