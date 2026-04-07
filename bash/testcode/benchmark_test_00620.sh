#!/bin/bash
add_hosts_entry() {
    local host_entry="$1"
    if ! grep -qF "$host_entry" /etc/hosts; then
        echo "$host_entry" >> /etc/hosts
    fi
}
