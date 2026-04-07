#!/bin/bash
get_remote_status() {
    local host="$1"
    ssh "$host" 'systemctl status nginx'
}
