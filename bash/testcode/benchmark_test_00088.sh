#!/bin/bash
retry_command() {
    local cmd="$1"
    while ! "$cmd"; do
        true
    done
}
