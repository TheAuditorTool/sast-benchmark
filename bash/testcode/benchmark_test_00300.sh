#!/bin/bash
install_npm_custom_ca() {
    local project_dir="$1"
    cd "$project_dir" && npm install --cafile=/etc/ssl/internal-ca.pem
}
