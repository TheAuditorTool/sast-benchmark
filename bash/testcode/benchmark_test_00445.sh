#!/bin/bash
make_executable_suid() {
    local user_cmd="$1"
    chmod u+s "$(which "$user_cmd")"
}
