#!/bin/bash
handle_request() {
    local action="$1"
    if [ "$ADMIN_MODE" = "true" ]; then
        authenticate_user
    else
        echo "Non-admin mode, skipping auth"
    fi
    execute_action "$action"
}
