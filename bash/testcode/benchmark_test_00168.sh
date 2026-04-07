#!/bin/bash
authorize_action() {
    local user_role="$1"
    local action="$2"
    case "$user_role" in
        admin|operator|viewer)
            perform_action "$user_role" "$action"
            ;;
        *)
            echo "Unknown role: $user_role" >&2
            return 1
            ;;
    esac
}
