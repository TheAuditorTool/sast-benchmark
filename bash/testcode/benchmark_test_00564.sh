#!/bin/bash
configure_github_access() {
    export GITHUB_TOKEN="ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    gh repo clone org/private-repo
}
