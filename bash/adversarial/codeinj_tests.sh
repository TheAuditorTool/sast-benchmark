#!/bin/bash
# Code Injection Test Cases (CWE-94)
# source/dot injection, bash -c, trap, heredoc expansion

# vuln-code-snippet start codeinj_source_user_path
load_user_plugin() {
    local plugin_path="$1"
    source "$plugin_path"  # vuln-code-snippet vuln-line codeinj_source_user_path
}
# vuln-code-snippet end codeinj_source_user_path

# vuln-code-snippet start codeinj_source_constant_safe
load_core_library() {
    local SCRIPT_DIR
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    source "${SCRIPT_DIR}/lib/utils.sh"  # vuln-code-snippet safe-line codeinj_source_constant_safe
}
# vuln-code-snippet end codeinj_source_constant_safe

# vuln-code-snippet start codeinj_source_partial_path
load_module_by_name() {
    local module_name="$1"
    local module_dir="/opt/app/modules"
    . "${module_dir}/${module_name}.sh"  # vuln-code-snippet vuln-line codeinj_source_partial_path
}
# vuln-code-snippet end codeinj_source_partial_path

# vuln-code-snippet start codeinj_source_validated_safe
load_env_config() {
    local env_name="$1"
    case "$env_name" in
        dev|staging|production)
            . "/etc/app/config/${env_name}.env"  # vuln-code-snippet safe-line codeinj_source_validated_safe
            ;;
        *)
            echo "Unknown environment: $env_name" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end codeinj_source_validated_safe

# vuln-code-snippet start codeinj_bash_c_injection
run_in_subshell() {
    local command_str="$1"
    bash -c "$command_str"  # vuln-code-snippet vuln-line codeinj_bash_c_injection
}
# vuln-code-snippet end codeinj_bash_c_injection

# vuln-code-snippet start codeinj_bash_c_literal_safe
run_health_check() {
    bash -c 'curl -sf http://localhost:8080/health > /dev/null && echo ok || echo fail'  # vuln-code-snippet safe-line codeinj_bash_c_literal_safe
}
# vuln-code-snippet end codeinj_bash_c_literal_safe

# vuln-code-snippet start codeinj_trap_injection
setup_cleanup_dynamic() {
    local cleanup_cmd="$1"
    trap "$cleanup_cmd" EXIT  # vuln-code-snippet vuln-line codeinj_trap_injection
}
# vuln-code-snippet end codeinj_trap_injection

# vuln-code-snippet start codeinj_trap_literal_safe
setup_cleanup_safe() {
    trap 'rm -f /tmp/lockfile; echo "Cleaned up"' EXIT  # vuln-code-snippet safe-line codeinj_trap_literal_safe
}
# vuln-code-snippet end codeinj_trap_literal_safe

# vuln-code-snippet start codeinj_heredoc_expansion
generate_script_dynamic() {
    local user_cmd="$1"
    cat << EOF > /tmp/generated.sh  # vuln-code-snippet vuln-line codeinj_heredoc_expansion
#!/bin/bash
$(${user_cmd})
EOF
    chmod +x /tmp/generated.sh
}
# vuln-code-snippet end codeinj_heredoc_expansion

# vuln-code-snippet start codeinj_heredoc_noquote_safe
generate_template_safe() {
    cat << 'EOF' > /tmp/template.sh
#!/bin/bash
echo "This is a static template"
echo "No variable expansion happens here"
EOF
    # vuln-code-snippet safe-line codeinj_heredoc_noquote_safe
    chmod +x /tmp/template.sh
}
# vuln-code-snippet end codeinj_heredoc_noquote_safe

# --- Tier 1 additions (Phase 2, verified 2026-03-19) ---

# vuln-code-snippet start codeinj_eval_cmd_substitution
install_remote_tool() {
    local url="$1"
    # Downloads remote content and executes via eval — NOT a pipe, so curl|bash
    # detection may miss this. Uses command substitution + eval instead.
    eval "$(curl -s "$url")"  # vuln-code-snippet vuln-line codeinj_eval_cmd_substitution
}
# vuln-code-snippet end codeinj_eval_cmd_substitution

# vuln-code-snippet start codeinj_eval_capture_only_safe
fetch_remote_info() {
    local url="$1"
    # Captures remote content but only displays it — no eval, no execution
    local output
    output=$(curl -s "$url")
    echo "$output"  # vuln-code-snippet safe-line codeinj_eval_capture_only_safe
}
# vuln-code-snippet end codeinj_eval_capture_only_safe

# vuln-code-snippet start codeinj_source_process_sub
load_remote_config() {
    local url="$1"
    # Process substitution creates fd, source reads it as bash code
    # Downloads and executes remote bash. Different mechanism than curl|bash pipe.
    source <(curl -s "$url")  # vuln-code-snippet vuln-line codeinj_source_process_sub
}
# vuln-code-snippet end codeinj_source_process_sub
