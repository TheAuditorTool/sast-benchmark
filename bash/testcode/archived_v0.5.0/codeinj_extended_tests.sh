#!/bin/bash
# Code Injection Extended Test Cases (CWE-94)
# Additional patterns beyond v0.4.0: eval variants, shell variable exec,
# download-then-source, stdin-to-bash, history re-exec, path component source.

# vuln-code-snippet start codeinj_mapfile_eval
load_dynamic_list() {
    local user_data="$1"
    # eval executes user_data as shell code. The process substitution <(...) captures
    # its stdout, but the eval itself runs arbitrary commands before mapfile reads output.
    # Any shell command in user_data executes with the script's privileges.
    mapfile -t lines < <(eval "$user_data")  # vuln-code-snippet vuln-line codeinj_mapfile_eval
}
# vuln-code-snippet end codeinj_mapfile_eval

# vuln-code-snippet start codeinj_assoc_array_eval
set_config_value() {
    local user_key="$1"
    local value="$2"
    # user_key is interpolated before eval processes the string. Injecting
    # 0$(id >&2) as user_key results in eval executing id during array index
    # evaluation — a confirmed bash behavior (declare -n nameref injection variant).
    eval "config[${user_key}]='${value}'"  # vuln-code-snippet vuln-line codeinj_assoc_array_eval
}
# vuln-code-snippet end codeinj_assoc_array_eval

# vuln-code-snippet start codeinj_shell_var_exec
run_user_script() {
    local user_script="$1"
    # $SHELL is an environment variable that the user controls. An attacker can set
    # SHELL=/usr/bin/python3 to change the interpreter, then craft user_script
    # accordingly. Additionally, user_script itself is unsanitized user input
    # directly passed as code.
    "$SHELL" -c "$user_script"  # vuln-code-snippet vuln-line codeinj_shell_var_exec
}
# vuln-code-snippet end codeinj_shell_var_exec

# vuln-code-snippet start codeinj_download_source
install_plugin() {
    local plugin_url="$1"
    local tmpfile
    tmpfile=$(mktemp)
    wget -q -O "$tmpfile" "$plugin_url"
    # The downloaded file is sourced without signature verification. An attacker who
    # controls plugin_url (or can intercept the download via MitM) delivers arbitrary
    # shell code that executes in the current process's context with full access to
    # all variables and functions.
    source "$tmpfile"  # vuln-code-snippet vuln-line codeinj_download_source
}
# vuln-code-snippet end codeinj_download_source

# vuln-code-snippet start codeinj_history_reexec
replay_last_command() {
    local user_cmd="$1"
    history -s "$user_cmd"
    # history -s adds user_cmd to bash's in-memory command history. fc -s re-executes
    # the most recently added history entry. In interactive bash sessions, this executes
    # user_cmd as shell code with no quoting — a complete code injection path.
    fc -s  # vuln-code-snippet vuln-line codeinj_history_reexec
}
# vuln-code-snippet end codeinj_history_reexec

# vuln-code-snippet start codeinj_stdin_to_bash
execute_user_code() {
    local user_code="$1"
    # bash reads commands from stdin when no script argument is given. Piping user_code
    # to bash stdin executes it as shell code. This is equivalent to eval in effect —
    # all shell commands in user_code run with the script's privileges.
    echo "$user_code" | bash  # vuln-code-snippet vuln-line codeinj_stdin_to_bash
}
# vuln-code-snippet end codeinj_stdin_to_bash

# vuln-code-snippet start codeinj_path_component_source
load_role_config() {
    local user_role="$1"
    # user_role is interpolated into the path. Supplying ../../etc/cron.d/malicious
    # as user_role traverses out of /etc/app/ and sources an attacker-controlled file.
    # The fixed prefix and suffix don't prevent traversal when the middle component
    # is user-supplied.
    source "/etc/app/${user_role}/config.sh"  # vuln-code-snippet vuln-line codeinj_path_component_source
}
# vuln-code-snippet end codeinj_path_component_source

# vuln-code-snippet start codeinj_bash_c_literal_string
announce_build_complete() {
    # The entire argument to bash -c is a single-quoted string literal. Single quotes
    # prevent all shell expansion. No user input is anywhere in scope. This is a
    # discrimination test: naive tools that flag any bash -c call will FP here.
    bash -c 'echo "build pipeline completed successfully"'  # vuln-code-snippet safe-line codeinj_bash_c_literal_string
}
# vuln-code-snippet end codeinj_bash_c_literal_string

# vuln-code-snippet start codeinj_proc_sub_hardcoded
configure_production_env() {
    # The process substitution <(...) contains only hardcoded echo commands with literal
    # strings. No user input reaches the sourced content. This tests whether tools
    # correctly trace that the process substitution has no taint flow.
    source <(echo "export APP_ENV=production"; echo "export LOG_LEVEL=warn")  # vuln-code-snippet safe-line codeinj_proc_sub_hardcoded
}
# vuln-code-snippet end codeinj_proc_sub_hardcoded

# vuln-code-snippet start codeinj_declare_p_own_var
export_array_to_subshell() {
    declare -A KNOWN_CONFIG=([key1]=val1 [key2]=val2 [key3]=val3)
    # declare -p serializes the variable to a valid bash assignment statement. Since
    # KNOWN_CONFIG is defined entirely within the script with no user input, the eval
    # executes only the known, controlled serialization. This is the standard bash
    # pattern for passing arrays to subshells.
    eval "$(declare -p KNOWN_CONFIG)"  # vuln-code-snippet safe-line codeinj_declare_p_own_var
}
# vuln-code-snippet end codeinj_declare_p_own_var

# vuln-code-snippet start codeinj_mapfile_fixed_path
load_approved_commands() {
    # The path is fully hardcoded. No eval is involved. mapfile reads line by line
    # from a trusted file into an array. No code execution path exists.
    mapfile -t COMMANDS < "/etc/app/approved_commands.txt"  # vuln-code-snippet safe-line codeinj_mapfile_fixed_path
}
# vuln-code-snippet end codeinj_mapfile_fixed_path

# vuln-code-snippet start codeinj_sanitized_env_source
load_system_profile() {
    # env -i starts bash with an empty environment, preventing environment variable
    # injection attacks ($IFS manipulation, $BASH_ENV, $ENV). The sourced path
    # /etc/app/system.conf is hardcoded. No user input is involved.
    env -i bash -c '. /etc/app/system.conf; env'  # vuln-code-snippet safe-line codeinj_sanitized_env_source
}
# vuln-code-snippet end codeinj_sanitized_env_source

# vuln-code-snippet start codeinj_case_dispatch
manage_service() {
    local cmd="$1"
    # Only three literal values (start, stop, reload) can reach the execution point.
    # The case statement is an allowlist: all other input is rejected. This tests
    # whether a SAST tool tracks the allowlist check upstream of the execution.
    case "$cmd" in
        start|stop|reload)
            "$cmd"  # vuln-code-snippet safe-line codeinj_case_dispatch
            ;;
        *)
            echo "Unknown command: $cmd" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end codeinj_case_dispatch

# vuln-code-snippet start codeinj_semver_export_eval
set_app_version() {
    local ver="$1"
    if [[ ! "$ver" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid version: $ver" >&2
        return 1
    fi
    # The regex ^[0-9]+\.[0-9]+\.[0-9]+$ permits only digits and dots. Neither shell
    # metacharacters ($, `, \, ', ", ;) nor word separators can appear in a string
    # matching this pattern. The eval executes a known-safe export statement:
    # eval "export APP_VERSION='1.2.3'" has no injection surface.
    eval "export APP_VERSION='${ver}'"  # vuln-code-snippet safe-line codeinj_semver_export_eval
}
# vuln-code-snippet end codeinj_semver_export_eval
