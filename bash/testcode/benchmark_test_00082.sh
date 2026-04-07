#!/bin/bash
run_as_root_su() {
    local user_cmd="$1"
    su -c "$user_cmd" root
}
