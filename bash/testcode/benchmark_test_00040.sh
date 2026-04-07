#!/bin/bash
log_user_login() {
    local username="$1"
    logger -p local0.info "User ${username} logged in successfully"
}
