#!/bin/bash
read_session_data() {
    local session_user="$1"
    local session_file="/tmp/session_${session_user}"
    if [ -f "$session_file" ]; then
        cat "$session_file"
    fi
}
