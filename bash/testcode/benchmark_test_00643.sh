#!/bin/bash
backup_user_file() {
    local user_file="$1"
    cp "$user_file" /var/backups/
}
