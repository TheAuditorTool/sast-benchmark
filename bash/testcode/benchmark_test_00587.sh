#!/bin/bash
setup_exit_handler() {
    trap cleanup EXIT
}
cleanup() {
    rm -f /tmp/app_lockfile
}
