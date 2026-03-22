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

# vuln-code-snippet start cmdi_eval_hardcoded
initialize_defaults() {
    local config_line="export APP_MODE=production"
    eval "$config_line"  # vuln-code-snippet safe-line cmdi_eval_hardcoded
}
# vuln-code-snippet end cmdi_eval_hardcoded

# vuln-code-snippet start cmdi_variable_as_command
dispatch_action() {
    local action="$1"
    shift
    $action "$@"  # vuln-code-snippet vuln-line cmdi_variable_as_command
}
# vuln-code-snippet end cmdi_variable_as_command

# vuln-code-snippet start cmdi_validated_command
dispatch_validated_action() {
    local action="$1"
    shift
    case "$action" in
        start|stop|restart|status)
            "$action" "$@"  # vuln-code-snippet safe-line cmdi_validated_command
            ;;
        *)
            echo "Invalid action: $action" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end cmdi_validated_command

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

# vuln-code-snippet start cmdi_arithmetic_validated
compute_offset() {
    local input="$1"
    if [[ ! "$input" =~ ^[0-9]+$ ]]; then
        echo "Invalid number" >&2
        return 1
    fi
    local result=$(( input + 1 ))  # vuln-code-snippet safe-line cmdi_arithmetic_validated
    echo "$result"
}
# vuln-code-snippet end cmdi_arithmetic_validated

# vuln-code-snippet start cmdi_xargs_tainted
process_file_list() {
    local user_list="$1"
    echo "$user_list" | xargs rm -f  # vuln-code-snippet vuln-line cmdi_xargs_tainted
}
# vuln-code-snippet end cmdi_xargs_tainted

# vuln-code-snippet start cmdi_xargs_null
process_file_list() {
    local file_list="$1"
    find "$file_list" -type f -print0 | xargs -0 rm -f  # vuln-code-snippet safe-line cmdi_xargs_null
}
# vuln-code-snippet end cmdi_xargs_null

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

# vuln-code-snippet start cmdi_find_exec_hardcoded
archive_old_logs() {
    local search_dir="$1"
    # Hardcoded command in -exec — no user control over what executes
    find "$search_dir" -name "*.log" -mtime +30 -exec gzip {} \;  # vuln-code-snippet safe-line cmdi_find_exec_hardcoded
}
# vuln-code-snippet end cmdi_find_exec_hardcoded

# vuln-code-snippet start cmdi_argument_injection_grep
search_logs() {
    local user_pattern="$1"
    local logfile="/var/log/app.log"
    # Attack: user_pattern="-e . /etc/shadow" reads arbitrary files
    # grep interprets leading dash as flag — no -- to prevent it
    grep "$user_pattern" "$logfile"  # vuln-code-snippet vuln-line cmdi_argument_injection_grep
}
# vuln-code-snippet end cmdi_argument_injection_grep

# vuln-code-snippet start cmdi_argument_injection_guarded
search_logs() {
    local user_pattern="$1"
    local logfile="/var/log/app.log"
    # Double dash prevents flag interpretation; -F treats pattern as literal string
    grep -F -- "$user_pattern" "$logfile"  # vuln-code-snippet safe-line cmdi_argument_injection_guarded
}
# vuln-code-snippet end cmdi_argument_injection_guarded

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

# vuln-code-snippet start cmdi_multistep_validated
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
    systemctl "$action" "$service"  # vuln-code-snippet safe-line cmdi_multistep_validated
}
# vuln-code-snippet end cmdi_multistep_validated

# vuln-code-snippet start cmdi_printf_q_sanitizer
safe_echo_input() {
    local input="$1"
    local escaped
    # printf %q shell-escapes all special characters
    escaped=$(printf '%q' "$input")
    # eval with escaped input is safe — all metacharacters are literal
    eval "echo $escaped"  # vuln-code-snippet safe-line cmdi_printf_q_sanitizer
}
# vuln-code-snippet end cmdi_printf_q_sanitizer

# [EXPECTED_FP] ${var@Q} is a bash 4.4+ quoting operator that produces single-quoted
# output safe for eval. Some SAST tools may not recognize ${var@Q} as a sanitizer.
# Engine will likely flag the eval usage without recognizing @Q as a sanitizer.
# vuln-code-snippet start cmdi_bash_qquote_sanitizer
safe_eval_with_qquote() {
    local input="$1"
    # ${var@Q} produces single-quoted version: all metacharacters are literal
    # Example: input="hello; rm -rf /" → ${input@Q} → 'hello; rm -rf /'
    local safe="${input@Q}"
    eval "echo $safe"  # vuln-code-snippet safe-line cmdi_bash_qquote_sanitizer
}
# vuln-code-snippet end cmdi_bash_qquote_sanitizer

# --- Phase 2 TN additions (OWASP 50/50 rebalancing, 2026-03-22) ---

# vuln-code-snippet start cmdi_dead_variable
process_with_constant() {
    #user input is read into $input but a hardcoded constant is used
    # at the eval sink. Tests whether the tool tracks data flow correctly.
    local input="$1"
    local cmd="date"
    eval "$cmd"  # vuln-code-snippet safe-line cmdi_dead_variable
}
# vuln-code-snippet end cmdi_dead_variable

# vuln-code-snippet start cmdi_echo_only
log_user_input() {
    #user input is only echoed to stdout. No subprocess, no eval,
    # no command execution. The tainted variable never reaches a command sink.
    local input="$1"
    echo "Received input: $input"  # vuln-code-snippet safe-line cmdi_echo_only
}
# vuln-code-snippet end cmdi_echo_only

# vuln-code-snippet start cmdi_ping_validated
check_host_status() {
    #hostname validated against strict regex before use as argument
    # to a hardcoded command. Only lowercase alphanumeric, dots, hyphens pass.
    local host="$1"
    if [[ ! "$host" =~ ^[a-z0-9][a-z0-9.-]*$ ]]; then
        echo "Invalid hostname" >&2
        return 1
    fi
    ping -c 1 "$host"  # vuln-code-snippet safe-line cmdi_ping_validated
}
# vuln-code-snippet end cmdi_ping_validated

# vuln-code-snippet start cmdi_head_integer
show_log_lines() {
    #line count coerced to integer via printf %d. Non-numeric input
    # results in 0 (harmless). The command (head) and file are hardcoded.
    local input="$1"
    local n
    printf -v n '%d' "$input" 2>/dev/null || n=10
    head -n "$n" /var/log/syslog  # vuln-code-snippet safe-line cmdi_head_integer
}
# vuln-code-snippet end cmdi_head_integer

# vuln-code-snippet start cmdi_exec_hardcoded
log_message_syslog() {
    #exec replaces the process with a hardcoded absolute-path binary.
    # User data is passed only as an argument to logger, never as a command.
    local message="$1"
    /usr/bin/logger -t app "$message"  # vuln-code-snippet safe-line cmdi_exec_hardcoded
}
# vuln-code-snippet end cmdi_exec_hardcoded

# vuln-code-snippet start cmdi_getopts
parse_deploy_options() {
    #getopts is a structured option parser built into bash.
    # It does not use eval or execute option values as commands.
    local environment="" version="" target=""
    while getopts "e:v:t:" opt; do
        case "$opt" in
            e) environment="$OPTARG" ;;
            v) version="$OPTARG" ;;
            t) target="$OPTARG" ;;
            *) return 1 ;;
        esac
    done
    echo "Deploy: env=$environment ver=$version target=$target"  # vuln-code-snippet safe-line cmdi_getopts
}
# vuln-code-snippet end cmdi_getopts

# vuln-code-snippet start cmdi_mapfile
load_host_list() {
    #mapfile -t reads lines into an array without shell interpretation.
    # The array is iterated with proper quoting. No eval, no word splitting.
    local file="$1"
    local -a hosts
    mapfile -t hosts < "$file"
    local host
    for host in "${hosts[@]}"; do
        echo "Host: $host"  # vuln-code-snippet safe-line cmdi_mapfile
    done
}
# vuln-code-snippet end cmdi_mapfile

# vuln-code-snippet start cmdi_ssh_singlequote
get_remote_status() {
    #remote command is single-quoted, preventing server-side expansion.
    # The host variable is the SSH target, not part of the executed command.
    local host="$1"
    ssh "$host" 'systemctl status nginx'  # vuln-code-snippet safe-line cmdi_ssh_singlequote
}
# vuln-code-snippet end cmdi_ssh_singlequote

# vuln-code-snippet start cmdi_docker_exec
check_container_health() {
    #docker exec command is single-quoted — no variable expansion
    # occurs inside the container. Container name is used only for targeting.
    local container="$1"
    docker exec "$container" sh -c 'cat /proc/loadavg'  # vuln-code-snippet safe-line cmdi_docker_exec
}
# vuln-code-snippet end cmdi_docker_exec

# vuln-code-snippet start cmdi_select_menu
interactive_service_control() {
    #select options are hardcoded string literals. The user picks
    # from a fixed menu — they cannot inject arbitrary commands.
    select opt in start stop status restart; do
        systemctl "$opt" app  # vuln-code-snippet safe-line cmdi_select_menu
        break
    done
}
# vuln-code-snippet end cmdi_select_menu

# vuln-code-snippet start cmdi_indirect_expansion
get_config_by_key() {
    #${!var} indirect expansion is used only after the variable name
    # is validated against a case allowlist of known config keys.
    local key="$1"
    local val
    case "$key" in
        DB_HOST|DB_PORT|APP_NAME|LOG_LEVEL)
            val="${!key}"  # vuln-code-snippet safe-line cmdi_indirect_expansion
            ;;
        *)
            echo "Unknown config key: $key" >&2
            return 1
            ;;
    esac
    echo "$val"
}
# vuln-code-snippet end cmdi_indirect_expansion

# vuln-code-snippet start cmdi_env_override
run_sandboxed() {
    #env -i clears the entire environment, preventing LD_PRELOAD,
    # PATH hijacking, and other environment-based attacks. The command
    # is a hardcoded absolute path (/usr/bin/date).
    env -i HOME="$HOME" PATH="/usr/bin:/bin" /usr/bin/date  # vuln-code-snippet safe-line cmdi_env_override
}
# vuln-code-snippet end cmdi_env_override

# vuln-code-snippet start cmdi_timeout_validated
run_with_timeout() {
    #timeout value is integer-validated. The command is a hardcoded
    # binary (curl). User URL is passed as a quoted argument.
    local secs="$1"
    local url="$2"
    if [[ ! "$secs" =~ ^[0-9]+$ ]]; then
        echo "Invalid timeout" >&2
        return 1
    fi
    timeout "$secs" /usr/bin/curl -sf "$url"  # vuln-code-snippet safe-line cmdi_timeout_validated
}
# vuln-code-snippet end cmdi_timeout_validated

# vuln-code-snippet start cmdi_basename_log
log_script_name() {
    #basename extracts the filename component of $0. The result
    # is used only in an echo for logging — never executed as a command.
    local name
    name=$(basename "$0")
    echo "Script: $name"  # vuln-code-snippet safe-line cmdi_basename_log
}
# vuln-code-snippet end cmdi_basename_log
