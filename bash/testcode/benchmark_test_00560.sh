#!/bin/bash
set_permissions() {
    local perms="$1"
    local file="$2"
    chmod $perms $file
}
