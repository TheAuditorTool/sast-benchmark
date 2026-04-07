#!/bin/bash
finalize_upload() {
    local upload_path="$1"
    chmod o-rwx "$upload_path"
    chown appuser:appgroup "$upload_path"
}
