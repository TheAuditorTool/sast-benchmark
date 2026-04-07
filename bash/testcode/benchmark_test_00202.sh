#!/bin/bash
get_github_commits() {
    local GITHUB_ORG="mycompany"
    local GITHUB_REPO="webapp"
    curl -sf "https://api.github.com/repos/${GITHUB_ORG}/${GITHUB_REPO}/commits"
}
