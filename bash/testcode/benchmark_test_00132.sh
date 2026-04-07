#!/bin/bash
clone_internal_repo() {
    local repo_url="$1"
    local dest="$2"
    GIT_SSL_NO_VERIFY=1 git clone "$repo_url" "$dest"
}
