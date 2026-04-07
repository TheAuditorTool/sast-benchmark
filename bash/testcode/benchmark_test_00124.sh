#!/bin/bash
read_user_report() {
    local user_path="$1"
    local canonical
    canonical=$(realpath -e "$user_path" 2>/dev/null) || { echo "Path not found" >&2; return 1; }
    if [[ "$canonical" != /var/app/reports/* ]]; then
        echo "Path outside allowed directory" >&2; return 1
    fi
    cat "$canonical"
}
