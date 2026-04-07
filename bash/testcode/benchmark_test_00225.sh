#!/bin/bash
validate_system_user() {
    local username="$1"
    local entry
    entry=$(getent shadow "$username" 2>&1)
    echo "Validation result for ${username}: ${entry}"
}
