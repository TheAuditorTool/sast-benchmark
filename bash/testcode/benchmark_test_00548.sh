#!/bin/bash
clone_with_env_bypass() {
    local repo_url="$1"
    local dest="$2"
    export GIT_SSL_NO_VERIFY=true
    git clone "$repo_url" "$dest"
}
