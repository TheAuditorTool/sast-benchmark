#!/bin/bash
acquire_mkdir_lock() {
    local lockdir="/tmp/app_run.lock"
    mkdir "$lockdir" 2>/dev/null && trap 'rmdir "$lockdir"' EXIT || { echo "Lock held" >&2; return 1; }
}
