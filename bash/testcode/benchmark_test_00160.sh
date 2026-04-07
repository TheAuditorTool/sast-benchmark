#!/bin/bash
sync_user_directory() {
    local user_src="$1"
    local dest="$2"
    rsync --recursive --delete "$user_src" "$dest"
}
