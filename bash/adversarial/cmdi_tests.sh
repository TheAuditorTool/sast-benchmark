#!/bin/bash
# Command Injection Test Cases (CWE-78)
# Mix of vulnerable and safe patterns for SAST benchmark
# Each function is one test case with vuln-code-snippet annotation

# vuln-code-snippet start cmdi_direct_eval
run_user_command() {
    local user_input="$1"
    eval "$user_input"  # vuln-code-snippet vuln-line cmdi_direct_eval
}
# vuln-code-snippet end cmdi_direct_eval

# vuln-code-snippet start cmdi_eval_hardcoded_safe
initialize_defaults() {
    local config_line="export APP_MODE=production"
    eval "$config_line"  # vuln-code-snippet safe-line cmdi_eval_hardcoded_safe
}
# vuln-code-snippet end cmdi_eval_hardcoded_safe

# vuln-code-snippet start cmdi_variable_as_command
dispatch_action() {
    local action="$1"
    shift
    $action "$@"  # vuln-code-snippet vuln-line cmdi_variable_as_command
}
# vuln-code-snippet end cmdi_variable_as_command

# vuln-code-snippet start cmdi_validated_command_safe
dispatch_validated_action() {
    local action="$1"
    shift
    case "$action" in
        start|stop|restart|status)
            "$action" "$@"  # vuln-code-snippet safe-line cmdi_validated_command_safe
            ;;
        *)
            echo "Invalid action: $action" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end cmdi_validated_command_safe

# vuln-code-snippet start cmdi_backtick_injection
get_dynamic_value() {
    local cmd="$1"
    local result=`$cmd`  # vuln-code-snippet vuln-line cmdi_backtick_injection
    echo "$result"
}
# vuln-code-snippet end cmdi_backtick_injection

# vuln-code-snippet start cmdi_nameref_injection
set_by_reference() {
    local ref_name="$1"
    local value="$2"
    declare -n ref="$ref_name"  # vuln-code-snippet vuln-line cmdi_nameref_injection
    ref="$value"
}
# vuln-code-snippet end cmdi_nameref_injection

# vuln-code-snippet start cmdi_arithmetic_expansion
compute_offset() {
    local user_expr="$1"
    local result=$(( user_expr ))  # vuln-code-snippet vuln-line cmdi_arithmetic_expansion
    echo "$result"
}
# vuln-code-snippet end cmdi_arithmetic_expansion

# vuln-code-snippet start cmdi_arithmetic_validated_safe
compute_offset_safe() {
    local input="$1"
    if [[ ! "$input" =~ ^[0-9]+$ ]]; then
        echo "Invalid number" >&2
        return 1
    fi
    local result=$(( input + 1 ))  # vuln-code-snippet safe-line cmdi_arithmetic_validated_safe
    echo "$result"
}
# vuln-code-snippet end cmdi_arithmetic_validated_safe

# vuln-code-snippet start cmdi_xargs_tainted
process_file_list() {
    local user_list="$1"
    echo "$user_list" | xargs rm -f  # vuln-code-snippet vuln-line cmdi_xargs_tainted
}
# vuln-code-snippet end cmdi_xargs_tainted

# vuln-code-snippet start cmdi_xargs_null_safe
process_file_list_safe() {
    local file_list="$1"
    find "$file_list" -type f -print0 | xargs -0 rm -f  # vuln-code-snippet safe-line cmdi_xargs_null_safe
}
# vuln-code-snippet end cmdi_xargs_null_safe

# vuln-code-snippet start cmdi_sed_expression_injection
replace_in_file() {
    local pattern="$1"
    local replacement="$2"
    local file="$3"
    sed -i "s/${pattern}/${replacement}/g" "$file"  # vuln-code-snippet vuln-line cmdi_sed_expression_injection
}
# vuln-code-snippet end cmdi_sed_expression_injection

# vuln-code-snippet start cmdi_awk_program_injection
run_awk_filter() {
    local awk_expr="$1"
    local file="$2"
    awk "$awk_expr" "$file"  # vuln-code-snippet vuln-line cmdi_awk_program_injection
}
# vuln-code-snippet end cmdi_awk_program_injection

# --- Tier 1 additions (Phase 2, verified 2026-03-19) ---

# vuln-code-snippet start cmdi_find_exec_injection
run_cleanup_on_files() {
    local cleanup_cmd="$1"
    local search_dir="$2"
    # Unquoted $cleanup_cmd splits into command + args for find -exec
    # Attack: cleanup_cmd="rm -rf" executes rm -rf on every matched file
    find "$search_dir" -name "*.tmp" -exec $cleanup_cmd {} \;  # vuln-code-snippet vuln-line cmdi_find_exec_injection
}
# vuln-code-snippet end cmdi_find_exec_injection

# vuln-code-snippet start cmdi_find_exec_hardcoded_safe
archive_old_logs() {
    local search_dir="$1"
    # Hardcoded command in -exec — no user control over what executes
    find "$search_dir" -name "*.log" -mtime +30 -exec gzip {} \;  # vuln-code-snippet safe-line cmdi_find_exec_hardcoded_safe
}
# vuln-code-snippet end cmdi_find_exec_hardcoded_safe

# vuln-code-snippet start cmdi_argument_injection_grep
search_logs() {
    local user_pattern="$1"
    local logfile="/var/log/app.log"
    # Attack: user_pattern="-e . /etc/shadow" reads arbitrary files
    # grep interprets leading dash as flag — no -- to prevent it
    grep "$user_pattern" "$logfile"  # vuln-code-snippet vuln-line cmdi_argument_injection_grep
}
# vuln-code-snippet end cmdi_argument_injection_grep

# vuln-code-snippet start cmdi_argument_injection_safe
search_logs_safe() {
    local user_pattern="$1"
    local logfile="/var/log/app.log"
    # Double dash prevents flag interpretation; -F treats pattern as literal string
    grep -F -- "$user_pattern" "$logfile"  # vuln-code-snippet safe-line cmdi_argument_injection_safe
}
# vuln-code-snippet end cmdi_argument_injection_safe

# vuln-code-snippet start cmdi_multistep_eval_chain
_build_deploy_cmd() {
    local service="$1"
    local action="$2"
    echo "systemctl ${action} ${service}"
}

execute_service_action() {
    local user_service="$1"
    local user_action="$2"
    # Taint flows: $1/$2 -> _build_deploy_cmd -> echo -> $cmd -> eval
    # Multi-hop: function boundary + string concatenation + eval
    local cmd
    cmd=$(_build_deploy_cmd "$user_service" "$user_action")
    eval "$cmd"  # vuln-code-snippet vuln-line cmdi_multistep_eval_chain
}
# vuln-code-snippet end cmdi_multistep_eval_chain

# vuln-code-snippet start cmdi_multistep_validated_safe
safe_service_control() {
    local service="$1"
    local action="$2"
    # Whitelist validation on BOTH inputs before use
    if [[ ! "$action" =~ ^(start|stop|restart|status)$ ]]; then
        echo "Invalid action" >&2
        return 1
    fi
    if [[ ! "$service" =~ ^[a-z][a-z0-9_.-]*$ ]]; then
        echo "Invalid service name" >&2
        return 1
    fi
    # Direct call — no eval, no string building
    systemctl "$action" "$service"  # vuln-code-snippet safe-line cmdi_multistep_validated_safe
}
# vuln-code-snippet end cmdi_multistep_validated_safe

# vuln-code-snippet start cmdi_printf_q_sanitizer_safe
safe_echo_input() {
    local input="$1"
    local escaped
    # printf %q shell-escapes all special characters
    escaped=$(printf '%q' "$input")
    # eval with escaped input is safe — all metacharacters are literal
    eval "echo $escaped"  # vuln-code-snippet safe-line cmdi_printf_q_sanitizer_safe
}
# vuln-code-snippet end cmdi_printf_q_sanitizer_safe

# [EXPECTED_FP] ${var@Q} is a bash 4.4+ quoting operator that produces single-quoted
# output safe for eval. Some SAST tools may not recognize ${var@Q} as a sanitizer.
# Engine will likely flag the eval usage without recognizing @Q as a sanitizer.
# vuln-code-snippet start cmdi_bash_qquote_sanitizer_safe
safe_eval_with_qquote() {
    local input="$1"
    # ${var@Q} produces single-quoted version: all metacharacters are literal
    # Example: input="hello; rm -rf /" → ${input@Q} → 'hello; rm -rf /'
    local safe="${input@Q}"
    eval "echo $safe"  # vuln-code-snippet safe-line cmdi_bash_qquote_sanitizer_safe
}
# vuln-code-snippet end cmdi_bash_qquote_sanitizer_safe
