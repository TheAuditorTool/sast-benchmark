#!/bin/bash
log_unit_event() {
    local user_unit_name="$1"
    escaped_name=$(systemd-escape "$user_unit_name")
    echo "Unit ${escaped_name} started" >> /var/log/units.log
}
