#!/bin/bash
watch_and_reload() {
    local config_file="$1"
    inotifywait -e close_write "$config_file"
    reload_service
}
