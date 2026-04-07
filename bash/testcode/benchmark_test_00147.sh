#!/bin/bash
setup_private_workspace() {
    local TMPDIR_PRIV
    TMPDIR_PRIV=$(mktemp -d)
    chmod 700 "$TMPDIR_PRIV"
}
