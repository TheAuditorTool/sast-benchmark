#!/bin/bash
# Remote Code Execution Test Cases (CWE-94)
# curl/wget pipe-to-shell patterns

# vuln-code-snippet start rce_curl_pipe_bash
install_tool_quick() {
    local url="$1"
    curl -sSL "$url" | bash  # vuln-code-snippet vuln-line rce_curl_pipe_bash
}
# vuln-code-snippet end rce_curl_pipe_bash

# vuln-code-snippet start rce_download_verify
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
    "$tmp_script"  # vuln-code-snippet safe-line rce_download_verify
    rm -f "$tmp_script"
}
# vuln-code-snippet end rce_download_verify

# vuln-code-snippet start rce_wget_pipe_sh
install_dependency() {
    local url="$1"
    wget -qO- "$url" | sh  # vuln-code-snippet vuln-line rce_wget_pipe_sh
}
# vuln-code-snippet end rce_wget_pipe_sh

# vuln-code-snippet start rce_curl_save_only
download_artifact() {
    local url="$1"
    local dest="$2"
    curl -sf -o "$dest" "$url"  # vuln-code-snippet safe-line rce_curl_save_only
    echo "Downloaded to $dest"
}
# vuln-code-snippet end rce_curl_save_only

# vuln-code-snippet start rce_process_substitution
run_remote_script() {
    local url="$1"
    bash <(curl -s "$url")  # vuln-code-snippet vuln-line rce_process_substitution
}
# vuln-code-snippet end rce_process_substitution
