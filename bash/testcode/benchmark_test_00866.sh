#!/bin/bash
switch_and_execute() {
    local target_user="$1"
    local command="$2"
    su - "$target_user" -c "$command"
}
