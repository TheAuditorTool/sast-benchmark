#!/bin/bash
stage_content() {
    local user_path="$1"
    rsync -av "${user_path}/" /tmp/staging/
}
