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

# vuln-code-snippet start rce_source_stdin_remote
load_remote_config() {
    local url="$1"
    # source /dev/stdin executes arbitrary code fetched from a remote URL
    # in the current shell — full access to all variables and functions.
    curl -sf "$url" | source /dev/stdin  # vuln-code-snippet vuln-line rce_source_stdin_remote
}
# vuln-code-snippet end rce_source_stdin_remote

# vuln-code-snippet start rce_eval_pkg_output
install_dependencies() {
    local manifest="$1"
    # eval on package manager output — attacker crafts a manifest that
    # causes the package manager to emit shell metacharacters.
    eval "$(pip install -r "$manifest" 2>&1)"  # vuln-code-snippet vuln-line rce_eval_pkg_output
}
# vuln-code-snippet end rce_eval_pkg_output

# vuln-code-snippet start rce_cron_from_url
install_cron_job() {
    local url="$1"
    # Downloads crontab entry from URL and installs directly — attacker
    # controls the cron schedule and command.
    curl -sf "$url" | crontab -  # vuln-code-snippet vuln-line rce_cron_from_url
}
# vuln-code-snippet end rce_cron_from_url

# vuln-code-snippet start rce_python_c_from_url
run_remote_python() {
    local url="$1"
    # Fetches Python code from URL and executes via python3 -c.
    local code
    code=$(curl -sf "$url")
    python3 -c "$code"  # vuln-code-snippet vuln-line rce_python_c_from_url
}
# vuln-code-snippet end rce_python_c_from_url

# vuln-code-snippet start rce_bash_heredoc_remote
execute_remote_heredoc() {
    local url="$1"
    # Fetches script content and pipes into bash via heredoc-style stdin.
    curl -sf "$url" | bash -s --  # vuln-code-snippet vuln-line rce_bash_heredoc_remote
}
# vuln-code-snippet end rce_bash_heredoc_remote

# --- Safe variants ---

# vuln-code-snippet start rce_gpg_verify_then_exec
install_signed_script() {
    local script_url="$1"
    local sig_url="$2"
    local tmp_script
    tmp_script=$(mktemp)
    local tmp_sig
    tmp_sig=$(mktemp)

    curl -sf -o "$tmp_script" "$script_url"
    curl -sf -o "$tmp_sig" "$sig_url"

    # GPG signature verification — rejects tampered scripts.
    if ! gpg --verify "$tmp_sig" "$tmp_script" 2>/dev/null; then
        rm -f "$tmp_script" "$tmp_sig"
        echo "GPG signature verification failed" >&2
        return 1
    fi

    chmod +x "$tmp_script"
    "$tmp_script"  # vuln-code-snippet safe-line rce_gpg_verify_then_exec
    rm -f "$tmp_script" "$tmp_sig"
}
# vuln-code-snippet end rce_gpg_verify_then_exec

# vuln-code-snippet start rce_noexec_quarantine
quarantine_download() {
    local url="$1"
    local quarantine_dir="/var/quarantine"
    # Downloaded to a noexec mount — cannot be executed even if attacker
    # tricks the script into running it.
    curl -sf -o "${quarantine_dir}/artifact" "$url"  # vuln-code-snippet safe-line rce_noexec_quarantine
    echo "Downloaded to quarantine (noexec mount)"
}
# vuln-code-snippet end rce_noexec_quarantine

# vuln-code-snippet start rce_package_manager_only
install_from_repo() {
    local package_name="$1"
    # Install via system package manager only — packages are signed and
    # verified by the distribution's GPG key infrastructure.
    if [[ "$package_name" =~ ^[a-z0-9][a-z0-9.+-]+$ ]]; then
        apt-get install -y "$package_name"  # vuln-code-snippet safe-line rce_package_manager_only
    else
        echo "Invalid package name" >&2
        return 1
    fi
}
# vuln-code-snippet end rce_package_manager_only

# vuln-code-snippet start rce_checksum_verify
install_verified_binary() {
    local url="$1"
    local expected_sha="$2"
    local dest="$3"

    curl -sf -o "$dest" "$url"

    # sha256sum --check verifies the downloaded file matches the expected hash.
    echo "${expected_sha}  ${dest}" | sha256sum --check --status  # vuln-code-snippet safe-line rce_checksum_verify
    if [[ $? -ne 0 ]]; then
        rm -f "$dest"
        echo "Checksum mismatch" >&2
        return 1
    fi
    chmod +x "$dest"
}
# vuln-code-snippet end rce_checksum_verify

# vuln-code-snippet start rce_local_script_only
run_local_plugin() {
    local plugin_name="$1"
    local plugin_dir="/usr/local/lib/myapp/plugins"
    # Only executes scripts from a trusted local directory — no remote fetch.
    local plugin_path="${plugin_dir}/${plugin_name}.sh"
    if [[ -f "$plugin_path" && -x "$plugin_path" ]]; then
        "$plugin_path"  # vuln-code-snippet safe-line rce_local_script_only
    else
        echo "Plugin not found or not executable" >&2
        return 1
    fi
}
# vuln-code-snippet end rce_local_script_only
