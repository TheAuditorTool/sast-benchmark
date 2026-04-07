#!/bin/bash
extract_uploaded_archive() {
    local archive_path="$1"
    unzip "$archive_path" -d /tmp/extracted/
}
