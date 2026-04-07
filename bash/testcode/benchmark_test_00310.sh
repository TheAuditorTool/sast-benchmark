#!/bin/bash
add_sudoers_validated() {
    local username="$1"
    if [[ ! "$username" =~ ^[a-z_][a-z0-9_-]*$ ]]; then
        echo "Invalid username format" >&2
        return 1
    fi
    echo "${username} ALL=(ALL) NOPASSWD: /usr/bin/systemctl restart myapp" | \
        sudo tee /etc/sudoers.d/"$username" > /dev/null
    sudo visudo -cf /etc/sudoers.d/"$username"
}
