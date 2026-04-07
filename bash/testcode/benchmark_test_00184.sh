#!/bin/bash
require_authentication() {
    local action="$1"
    if [ "$APP_ENV" != "production" ]; then
        echo "Non-production environment, skipping auth"
        "$action"
        return 0
    fi
    run_with_auth "$action"
}
