#!/bin/bash
# Code Injection Test Cases (CWE-94)
# source/dot injection, bash -c, trap, heredoc expansion

# vuln-code-snippet start codeinj_source_user_path
load_user_plugin() {
    local plugin_path="$1"
    source "$plugin_path"  # vuln-code-snippet vuln-line codeinj_source_user_path
}
# vuln-code-snippet end codeinj_source_user_path

# vuln-code-snippet start codeinj_source_constant
load_core_library() {
    local SCRIPT_DIR
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    source "${SCRIPT_DIR}/lib/utils.sh"  # vuln-code-snippet safe-line codeinj_source_constant
}
# vuln-code-snippet end codeinj_source_constant

# vuln-code-snippet start codeinj_source_partial_path
load_module_by_name() {
    local module_name="$1"
    local module_dir="/opt/app/modules"
    . "${module_dir}/${module_name}.sh"  # vuln-code-snippet vuln-line codeinj_source_partial_path
}
# vuln-code-snippet end codeinj_source_partial_path

# vuln-code-snippet start codeinj_source_validated
load_env_config() {
    local env_name="$1"
    case "$env_name" in
        dev|staging|production)
            . "/etc/app/config/${env_name}.env"  # vuln-code-snippet safe-line codeinj_source_validated
            ;;
        *)
            echo "Unknown environment: $env_name" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end codeinj_source_validated

# vuln-code-snippet start codeinj_bash_c_injection
run_in_subshell() {
    local command_str="$1"
    bash -c "$command_str"  # vuln-code-snippet vuln-line codeinj_bash_c_injection
}
# vuln-code-snippet end codeinj_bash_c_injection

# vuln-code-snippet start codeinj_bash_c_literal
run_health_check() {
    bash -c 'curl -sf http://localhost:8080/health > /dev/null && echo ok || echo fail'  # vuln-code-snippet safe-line codeinj_bash_c_literal
}
# vuln-code-snippet end codeinj_bash_c_literal

# vuln-code-snippet start codeinj_trap_injection
setup_cleanup_dynamic() {
    local cleanup_cmd="$1"
    trap "$cleanup_cmd" EXIT  # vuln-code-snippet vuln-line codeinj_trap_injection
}
# vuln-code-snippet end codeinj_trap_injection

# vuln-code-snippet start codeinj_trap_literal
setup_cleanup() {
    trap 'rm -f /tmp/lockfile; echo "Cleaned up"' EXIT  # vuln-code-snippet safe-line codeinj_trap_literal
}
# vuln-code-snippet end codeinj_trap_literal

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

# vuln-code-snippet start codeinj_heredoc_noquote
generate_template() {
    cat << 'EOF' > /tmp/template.sh
#!/bin/bash
echo "This is a static template"
echo "No variable expansion happens here"
EOF
    # vuln-code-snippet safe-line codeinj_heredoc_noquote
    chmod +x /tmp/template.sh
}
# vuln-code-snippet end codeinj_heredoc_noquote

# --- Tier 1 additions (Phase 2, verified 2026-03-19) ---

# vuln-code-snippet start codeinj_eval_cmd_substitution
install_remote_tool() {
    local url="$1"
    # Downloads remote content and executes via eval — NOT a pipe, so curl|bash
    # detection may miss this. Uses command substitution + eval instead.
    eval "$(curl -s "$url")"  # vuln-code-snippet vuln-line codeinj_eval_cmd_substitution
}
# vuln-code-snippet end codeinj_eval_cmd_substitution

# vuln-code-snippet start codeinj_eval_capture_only
fetch_remote_info() {
    local url="$1"
    # Captures remote content but only displays it — no eval, no execution
    local output
    output=$(curl -s "$url")
    echo "$output"  # vuln-code-snippet safe-line codeinj_eval_capture_only
}
# vuln-code-snippet end codeinj_eval_capture_only

# vuln-code-snippet start codeinj_source_process_sub
load_remote_config() {
    local url="$1"
    # Process substitution creates fd, source reads it as bash code
    # Downloads and executes remote bash. Different mechanism than curl|bash pipe.
    source <(curl -s "$url")  # vuln-code-snippet vuln-line codeinj_source_process_sub
}
# vuln-code-snippet end codeinj_source_process_sub

# --- Phase 2 TN additions (OWASP 50/50 rebalancing, 2026-03-22) ---

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# vuln-code-snippet start codeinj_source_regex_dir
load_lib() {
    #library name validated against strict regex (lowercase alpha + digits
    # + underscore only) before being used in source path.
    local name="$1"
    if [[ ! "$name" =~ ^[a-z][a-z0-9_]+$ ]]; then
        echo "Invalid library name: $name" >&2
        return 1
    fi
    source "${SCRIPT_DIR}/lib/${name}.sh"  # vuln-code-snippet safe-line codeinj_source_regex_dir
}
# vuln-code-snippet end codeinj_source_regex_dir

# vuln-code-snippet start codeinj_bash_c_static
run_db_maintenance() {
    #bash -c with single-quoted argument. Single quotes prevent all
    # variable expansion — the command is a fixed string literal.
    local db_path="$1"
    bash -c 'sqlite3 /var/app/app.db "VACUUM; ANALYZE"'  # vuln-code-snippet safe-line codeinj_bash_c_static
}
# vuln-code-snippet end codeinj_bash_c_static

# vuln-code-snippet start codeinj_printf_integer
format_numeric_value() {
    #printf -v assigns an integer representation directly to $result.
    # No eval is used. Non-numeric input produces 0 (harmless).
    local input="$1"
    local result
    printf -v result '%d' "$input" 2>/dev/null  # vuln-code-snippet safe-line codeinj_printf_integer
    echo "Formatted: $result"
}
# vuln-code-snippet end codeinj_printf_integer

# vuln-code-snippet start codeinj_process_sub_read
read_config_entries() {
    #process substitution is used for READING data (grep output),
    # not for sourcing/executing code. The config file content is never
    # interpreted as bash commands.
    local config_file="$1"
    local key value
    while IFS='=' read -r key value; do
        echo "Config: $key = $value"  # vuln-code-snippet safe-line codeinj_process_sub_read
    done < <(grep -v '^#' "$config_file")
}
# vuln-code-snippet end codeinj_process_sub_read

# vuln-code-snippet start codeinj_source_feature
activate_feature() {
    #feature name validated against strict regex before path construction.
    # Only lowercase letters and underscores allowed — prevents path traversal
    # and ensures only legitimate feature flag files can be sourced.
    local feature="$1"
    if [[ ! "$feature" =~ ^[a-z_]+$ ]]; then
        echo "Invalid feature name: $feature" >&2
        return 1
    fi
    source "/etc/app/features/${feature}.sh"  # vuln-code-snippet safe-line codeinj_source_feature
}
# vuln-code-snippet end codeinj_source_feature

# vuln-code-snippet start codeinj_grep_metadata
read_plugin_metadata() {
    #plugin file is parsed with grep/cut for declarative metadata.
    # The file is NEVER sourced — content is not executed as bash code.
    local plugin_file="$1"
    local version name
    version=$(grep '^VERSION=' "$plugin_file" | head -1 | cut -d= -f2)
    name=$(grep '^NAME=' "$plugin_file" | head -1 | cut -d= -f2)
    echo "Plugin: $name v$version"  # vuln-code-snippet safe-line codeinj_grep_metadata
}
# vuln-code-snippet end codeinj_grep_metadata

# vuln-code-snippet start codeinj_trap_funcref
setup_exit_handler() {
    #trap uses a function NAME reference, not a string to be eval'd.
    # Bash calls the named function directly — no string interpretation occurs.
    # This is distinct from trap "$string_cmd" which evaluates the string.
    trap cleanup EXIT  # vuln-code-snippet safe-line codeinj_trap_funcref
}

cleanup() {
    rm -f /tmp/app_lockfile
}
# vuln-code-snippet end codeinj_trap_funcref

# vuln-code-snippet start codeinj_declare_f_dispatch
call_handler() {
    #function existence verified via declare -F before calling.
    # Only functions already defined in the current shell can be dispatched.
    # An attacker cannot inject a new function name — it must already exist.
    local fn="$1"
    shift
    if declare -F "$fn" > /dev/null 2>&1; then
        "$fn" "$@"  # vuln-code-snippet safe-line codeinj_declare_f_dispatch
    else
        echo "Handler not found: $fn" >&2
        return 1
    fi
}
# vuln-code-snippet end codeinj_declare_f_dispatch
