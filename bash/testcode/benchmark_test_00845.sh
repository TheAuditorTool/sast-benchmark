#!/bin/bash
compute_offset() {
    local user_expr="$1"
    local result=$(( user_expr ))
    echo "$result"
}
