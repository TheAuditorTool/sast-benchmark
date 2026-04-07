#!/bin/bash
load_session() {
    local session_id="$1"
    local session_file="/var/app/sessions/${session_id}"
    local perms
    perms=$(stat -c '%a' "$session_file" 2>/dev/null)
    if [[ "$perms" != "600" ]]; then
        echo "Session file has wrong permissions, rejecting" >&2; return 1
    fi
    cat "$session_file"
}
