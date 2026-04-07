#!/bin/bash
replay_last_command() {
    local user_cmd="$1"
    history -s "$user_cmd"
    fc -s
}
