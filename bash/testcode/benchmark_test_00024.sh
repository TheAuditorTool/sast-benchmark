#!/bin/bash
log_user_event() {
    local user_tag="$1"
    local user_msg="$2"
    systemd-cat -t "$user_tag" <<< "$user_msg"
}
