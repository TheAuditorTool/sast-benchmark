#!/bin/bash
install_npm_packages() {
    local project_dir="$1"
    cd "$project_dir" && npm install --strict-ssl=false
}
