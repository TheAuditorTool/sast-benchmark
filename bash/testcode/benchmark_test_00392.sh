#!/bin/bash
verify_expected_user() {
    local expected_user="$1"
    local actual
    actual=$(logname)
    [ "$actual" = "$expected_user" ]
}
