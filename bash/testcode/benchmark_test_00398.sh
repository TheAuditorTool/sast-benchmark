#!/bin/bash
run_in_user_namespace() {
    local cmd="$1"
    unshare --map-root-user bash -c "$cmd"
}
