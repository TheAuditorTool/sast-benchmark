#!/bin/bash
manage_service() {
    local cmd="$1"
    case "$cmd" in
        start|stop|reload)
            "$cmd"
            ;;
        *)
            echo "Unknown command: $cmd" >&2
            return 1
            ;;
    esac
}
