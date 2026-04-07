#!/bin/bash
create_htpasswd_file() {
    local user="$1"
    htpasswd -bc /etc/nginx/.htpasswd "$user" "WebAdmin2025!"
}
