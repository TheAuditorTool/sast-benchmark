#!/bin/bash
deploy_plugin_from_repo() {
    local repo_url="$1"
    git clone "$repo_url" /tmp/plugin_src
    bash /tmp/plugin_src/install.sh
}
