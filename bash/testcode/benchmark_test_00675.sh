#!/bin/bash
store_session_data() {
    local session_data="$1"
    echo "$session_data" > /dev/shm/app_session
}
