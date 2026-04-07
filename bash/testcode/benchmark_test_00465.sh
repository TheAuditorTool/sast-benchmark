#!/bin/bash
create_workspace() {
    local WORK_DIR
    WORK_DIR=$(mktemp -d)
    trap "rm -rf '$WORK_DIR'" EXIT
    echo "Workspace: $WORK_DIR"
}
