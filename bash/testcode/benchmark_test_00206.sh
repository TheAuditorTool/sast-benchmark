#!/bin/bash
sync_files_daemon() {
    local src="$1"
    local dest_host="$2"
    RSYNC_PASSWORD=SyncPass rsync "$src" "rsync://${dest_host}/backups/"
}
