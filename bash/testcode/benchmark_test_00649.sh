#!/bin/bash
update_system_time() {
    local ntp_server="$1"
    sudo /usr/sbin/ntpdate "$ntp_server"
}
