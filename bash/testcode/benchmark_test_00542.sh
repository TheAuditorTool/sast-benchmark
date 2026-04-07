#!/bin/bash
deploy_static_content() {
    local html_file="$1"
    cp "$html_file" /var/www/html/
    chmod 644 /var/www/html/"$(basename "$html_file")"
}
