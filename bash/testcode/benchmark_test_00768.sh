#!/bin/bash
extract_cpio_archive() {
    local archive_path="$1"
    cpio -idv < "$archive_path"
}
