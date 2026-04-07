#!/bin/bash
handle_github_webhook() {
    local body="$1"
    local ref
    ref=$(echo "$body" | jq -r '.ref')
    local branch="${ref#refs/heads/}"
    deploy_branch "$branch"
}
