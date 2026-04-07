#!/bin/bash
create_work_dir_respecting_tmpdir() {
    local TMPDIR_WORK
    TMPDIR_WORK=$(mktemp -d "${TMPDIR:-/tmp}/app.XXXXXXXXXX")
    echo "$TMPDIR_WORK"
}
