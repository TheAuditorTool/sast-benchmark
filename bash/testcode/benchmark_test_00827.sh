#!/bin/bash
sync_files_encrypted() {
    local src="$1"
    local dest_host="$2"
    rsync -az -e ssh "$src" "${dest_host}:/backups/"
}
