#!/bin/bash
install_signed_script() {
    local script_url="$1"
    local sig_url="$2"
    local tmp_script
    tmp_script=$(mktemp)
    local tmp_sig
    tmp_sig=$(mktemp)
    curl -sf -o "$tmp_script" "$script_url"
    curl -sf -o "$tmp_sig" "$sig_url"
    if ! gpg --verify "$tmp_sig" "$tmp_script" 2>/dev/null; then
        rm -f "$tmp_script" "$tmp_sig"
        echo "GPG signature verification failed" >&2
        return 1
    fi
    chmod +x "$tmp_script"
    "$tmp_script"
    rm -f "$tmp_script" "$tmp_sig"
}
