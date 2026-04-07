#!/bin/bash
dispatch_action() {
    local action="$1"
    shift
    $action "$@"
}
