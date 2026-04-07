#!/bin/bash
grant_sudo_access() {
    local username="$1"
    echo "${username} ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
}
