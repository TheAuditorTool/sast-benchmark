#!/bin/bash
install_tool_verified() {
    local url="$1"
    local expected_hash="$2"
    local tmp_script
    tmp_script=$(mktemp)
    curl -sf -o "$tmp_script" "$url"
    local actual_hash
    actual_hash=$(sha256sum "$tmp_script" | awk '{print $1}')
    if [[ "$actual_hash" != "$expected_hash" ]]; then
        rm -f "$tmp_script"
        echo "Checksum mismatch — aborting" >&2
        return 1
    fi
    chmod +x "$tmp_script"
    "$tmp_script"
    rm -f "$tmp_script"
}
