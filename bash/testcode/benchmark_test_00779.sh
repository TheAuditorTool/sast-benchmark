#!/bin/bash
pull_private_repo() {
    local repo="$1"
    local branch="${2:-main}"
    git -c http.sslVerify=false clone --branch "$branch" "$repo"
}
