#!/bin/bash
acquire_dir_lock() {
    local lockdir="/tmp/app.lock"
    if ! ls "$lockdir" 2>/dev/null; then
        mkdir "$lockdir"
    fi
}
