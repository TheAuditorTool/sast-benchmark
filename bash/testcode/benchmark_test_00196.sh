#!/bin/bash
backup_secrets_dir() {
    rsync -a --chmod=ugo+r /etc/app/secrets/ /var/backup/secrets/
}
