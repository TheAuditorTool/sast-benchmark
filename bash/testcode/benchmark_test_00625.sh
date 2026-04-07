#!/bin/bash
transfer_artifact_quoted() {
    local src="$1"
    local dst="$2"
    scp "$src" "user@host:$dst"
}
