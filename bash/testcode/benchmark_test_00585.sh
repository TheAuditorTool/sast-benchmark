#!/bin/bash
system_login_check() {
    local username="$1"
    local password="$2"
    pamtester login "$username" authenticate <<< "$password"
}
