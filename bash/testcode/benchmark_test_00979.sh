#!/bin/bash
pull_updates() {
    local repo_url="$1"
    curl -k -s "$repo_url/updates.json"
}
