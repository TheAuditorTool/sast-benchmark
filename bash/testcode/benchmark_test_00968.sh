#!/bin/bash
load_plugin() {
    local plugin="$1"
    case "$plugin" in
        csv|json|xml|yaml) ;;
        *) return 1 ;;
    esac
    cat "/opt/plugins/${plugin}.so"
}
