#!/bin/bash
set_permissions_quoted() {
    local perms="$1"
    local file="$2"
    chmod "$perms" "$file"
}
