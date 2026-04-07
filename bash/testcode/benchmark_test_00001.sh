#!/bin/bash
upload_backup() {
    local file="$1"
    local host="$2"
    curl -T "$file" "ftp://backup_user:Backup2025@${host}/backups/"
}
