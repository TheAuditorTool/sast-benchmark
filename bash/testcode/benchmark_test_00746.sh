#!/bin/bash
reset_work_directory() {
    local WORK_DIR="/tmp/work"
    rm -rf "$WORK_DIR"
    mkdir "$WORK_DIR"
}
