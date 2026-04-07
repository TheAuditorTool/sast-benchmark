#!/bin/bash
dispatch_validated_action() {
    local action="$1"
    shift
    case "$action" in
        start|stop|restart|status)
            "$action" "$@"
            ;;
        *)
            echo "Invalid action: $action" >&2
            return 1
            ;;
    esac
}
