#!/bin/bash
execute_as_root() {
    local cmd="$1"
    runuser -l root -c "$cmd"
}
