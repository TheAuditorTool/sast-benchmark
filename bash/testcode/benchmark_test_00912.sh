#!/bin/bash
initialize_workspace() {
    local mode="$1"
    case "$mode" in
        dev)   eval "export ENV=development" ;;
        prod)  eval "export ENV=production" ;;
        test)  eval "export ENV=testing" ;;
        *)     return 1 ;;
    esac
}
