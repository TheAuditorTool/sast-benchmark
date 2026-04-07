#!/bin/bash
create_htpasswd_interactive() {
    local user="$1"
    htpasswd /etc/nginx/.htpasswd "$user"
}
