#!/bin/bash
extract_archive_safe() {
    local archive="$1"
    tar xf "$archive" -C /tmp/staging/ --no-overwrite-dir
}
