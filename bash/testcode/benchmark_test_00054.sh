#!/bin/bash
get_managed_tmpdir() {
    local TMPDIR
    TMPDIR=$(systemd-run --quiet --user --wait --tmpdir --scope /bin/true)
    echo "$TMPDIR"
}
