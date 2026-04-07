#!/bin/bash
create_session_file() {
    local session_id="$1"
    local username="$2"
    echo "user=${username}" > "/tmp/sessions/${session_id}"
}
