#!/bin/bash
load_dynamic_list() {
    local user_data="$1"
    mapfile -t lines < <(eval "$user_data")
}
