#!/bin/bash
add_sudoers_rule() {
    local username="$1"
    echo "${username} ALL=(ALL) NOPASSWD: /usr/bin/systemctl" >> /etc/sudoers
}
