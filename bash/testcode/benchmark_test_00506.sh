#!/bin/bash
archive_with_no_paths() {
    local user_file="$1"
    zip -j /tmp/archive.zip "$user_file"
}
