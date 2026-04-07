#!/bin/bash
check_repo_refs() {
    local repo_url="$1"
    git ls-remote "$repo_url"
}
