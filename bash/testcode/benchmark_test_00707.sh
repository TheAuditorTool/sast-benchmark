#!/bin/bash
log_restricted_entry() {
    local log_msg="$1"
    ( umask 027; echo "$log_msg" | tee -a /var/log/app.log )
}
