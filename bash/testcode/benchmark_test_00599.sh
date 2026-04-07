#!/bin/bash
upload_backup_encrypted() {
    local file="$1"
    local host="$2"
    sftp "backup_user@${host}" <<< "put ${file} /backups/"
}
