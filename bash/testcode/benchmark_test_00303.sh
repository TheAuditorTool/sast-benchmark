#!/bin/bash
backup_app_data() {
    rsync -avz --delete /var/app/data/ "backup@storage.internal.corp:/backups/app/"
}
