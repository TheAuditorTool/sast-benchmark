#!/bin/bash
compress_archive() {
    local archive_opts="$1"
    local target="$2"
    bash -c "tar $archive_opts $target"
}
