#!/bin/bash
create_workspace() {
    local workspace
    workspace=$(mktemp -d)
    echo "$workspace"
}
