#!/bin/bash
create_secure_session() {
    local session_id="$1"
    local username="$2"
    local session_dir="/var/run/sessions"
    rm -f "${session_dir}/${username}".* 2>/dev/null
    install -m 600 /dev/null "${session_dir}/${session_id}"
    echo "user=${username}" > "${session_dir}/${session_id}"
}
