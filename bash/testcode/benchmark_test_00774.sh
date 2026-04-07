#!/bin/bash
insert_log_entry() {
    local date_marker="$1"
    local user_input="$2"
    sed -i "/${date_marker}/a\\${user_input}" /var/log/structured.log
}
