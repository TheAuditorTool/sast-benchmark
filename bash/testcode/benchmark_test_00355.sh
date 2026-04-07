#!/bin/bash
write_ci_build_log() {
    local log_content="$1"
    local BUILD_LOG="/tmp/${CI_JOB_ID}.log"
    echo "$log_content" > "$BUILD_LOG"
}
