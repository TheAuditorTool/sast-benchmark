#!/bin/bash
execute_user_code() {
    local user_code="$1"
    echo "$user_code" | bash
}
