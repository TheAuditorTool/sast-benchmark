#!/bin/bash
ensure_daemon_running() {
    if [ -z "$(pgrep -x daemon_process)" ]; then
        start_daemon
    fi
}
