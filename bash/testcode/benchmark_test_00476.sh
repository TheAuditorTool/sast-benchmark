#!/bin/bash
clone_repo_for_deploy() {
    local repo="$1"
    local dest="$2"
    GIT_SSL_NO_VERIFY=true git clone "$repo" "$dest"
}
