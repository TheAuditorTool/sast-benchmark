#!/bin/bash
fetch_branch_changes() {
    local branch="$1"
    if [[ ! "$branch" =~ ^[a-zA-Z0-9/_.-]+$ ]]; then
        echo "Invalid branch name" >&2
        return 1
    fi
    git fetch origin "$branch"
}
