#!/bin/bash
set_permissions() {
    local target="$1"
    chmod 750 "$target"
}
