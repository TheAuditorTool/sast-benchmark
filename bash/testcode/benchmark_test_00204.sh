#!/bin/bash
run_on_hosts() {
    local hosts_file="$1"
    local user_cmd="$2"
    ansible -i "$hosts_file" all -m shell -a "$user_cmd"
}
