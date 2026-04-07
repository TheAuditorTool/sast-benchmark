#!/bin/bash
load_verified_config() {
    local cfg="$1"
    if stat --format='%U' "$cfg" 2>/dev/null | grep -q "^root$"; then
        source "$cfg"
    else
        echo "Config not owned by root" >&2
        return 1
    fi
}
