#!/bin/bash
# Information Disclosure Extended Test Cases (CWE-200/532)
# Additional patterns beyond v0.4.0: credentials in error messages, process list
# exposure, env dumps to web roots, verbose curl logging, set -x with creds,
# strace logging, and extended safe/filtered variants.

# vuln-code-snippet start infodisclosure_creds_in_error
connect_to_database() {
    local db_host="$1"
    local db_user="$2"
    local db_pass="$3"
    # The error message reveals the database hostname, username, AND password to
    # stderr. Even stderr output may be captured in log aggregators, monitoring
    # dashboards, or CI/CD pipeline logs. Credential leakage in error messages
    # is a common pathway for secret exposure.
    mysql -h "$db_host" -u "$db_user" -p"$db_pass" appdb -e "SELECT 1" 2>/dev/null || \
        echo "Error: cannot connect to ${db_host} with user ${db_user}:${db_pass}" >&2  # vuln-code-snippet vuln-line infodisclosure_creds_in_error
}
# vuln-code-snippet end infodisclosure_creds_in_error

# vuln-code-snippet start infodisclosure_ps_to_email
send_process_report() {
    local admin_email="$1"
    # ps aux shows command-line arguments of all processes. Any process that
    # passes credentials as arguments (mysql -p$pass, curl -H "Authorization:
    # Bearer $token") is exposed in the process listing. Emailing ps output
    # distributes this sensitive data outside the system.
    ps aux | mail -s "Process list report" "$admin_email"  # vuln-code-snippet vuln-line infodisclosure_ps_to_email
}
# vuln-code-snippet end infodisclosure_ps_to_email

# vuln-code-snippet start infodisclosure_env_to_webroot
dump_debug_info() {
    # The environment dump is written to a web-accessible directory. Any
    # variable in the environment — including DB_PASS, API_KEY,
    # AWS_SECRET_ACCESS_KEY — is immediately readable by unauthenticated
    # HTTP clients.
    env > /var/www/html/debug.txt  # vuln-code-snippet vuln-line infodisclosure_env_to_webroot
}
# vuln-code-snippet end infodisclosure_env_to_webroot

# vuln-code-snippet start infodisclosure_curl_verbose_log
call_api_with_logging() {
    local api_token="$1"
    # curl -v prints request and response headers to stderr. 2>&1 merges this
    # with stdout. tee writes the merged output to a log file. The
    # Authorization: Bearer header (containing the token) appears verbatim in
    # /var/log/api_debug.log.
    curl -v -H "Authorization: Bearer ${api_token}" \
        "https://api.example.com/v1/data" 2>&1 | tee /var/log/api_debug.log  # vuln-code-snippet vuln-line infodisclosure_curl_verbose_log
}
# vuln-code-snippet end infodisclosure_curl_verbose_log

# vuln-code-snippet start infodisclosure_set_x_with_creds
run_db_migration() {
    local db_pass="$1"
    # set -x enables bash execution tracing — every command is printed to
    # stderr prefixed with +. The mysql command including -p"$db_pass" is
    # traced to stderr with the password visible in plaintext. Shell tracing
    # output is often captured by logging infrastructure.
    set -x  # vuln-code-snippet vuln-line infodisclosure_set_x_with_creds
    mysql -h db.internal.corp -uapp -p"$db_pass" appdb -e "source /var/app/migration.sql"
    set +x
}
# vuln-code-snippet end infodisclosure_set_x_with_creds

# vuln-code-snippet start infodisclosure_secrets_tar
create_debug_bundle() {
    # The debug archive includes /var/secrets/ and /home/app/.env which contain
    # production credentials. The archive is written to /tmp with a predictable
    # timestamp-based name, accessible to all local users. Debug bundles must
    # never include secrets directories.
    tar czf "/tmp/debug_$(date +%s).tar.gz" /etc/app/ /var/secrets/ /home/app/.env  # vuln-code-snippet vuln-line infodisclosure_secrets_tar
}
# vuln-code-snippet end infodisclosure_secrets_tar

# vuln-code-snippet start infodisclosure_python_env_print
debug_environment() {
    # Printing the full environment dictionary reveals all environment variables
    # including DB_PASSWORD, AWS_SECRET_ACCESS_KEY, and any other secrets
    # injected via environment. The output goes to stdout where it can be
    # captured by log aggregation.
    python3 -c "import os; print(dict(os.environ))"  # vuln-code-snippet vuln-line infodisclosure_python_env_print
}
# vuln-code-snippet end infodisclosure_python_env_print

# vuln-code-snippet start infodisclosure_script_args_log
log_startup_info() {
    # ${*} expands to all positional parameters. If the script is called with
    # credentials as arguments (e.g., ./script.sh --password=secret), they are
    # logged. Command-line arguments also appear in ps aux and shell history.
    echo "Starting ${0} with arguments: ${*}"  # vuln-code-snippet vuln-line infodisclosure_script_args_log
}
# vuln-code-snippet end infodisclosure_script_args_log

# vuln-code-snippet start infodisclosure_df_to_webroot
publish_system_status() {
    # Disk usage information reveals filesystem mount points, partition names,
    # and available space. This can expose internal storage architecture (NFS
    # mounts, logical volume names, encrypted partition existence) to
    # unauthenticated web clients.
    df -h | awk '{print}' >> /var/www/html/status.txt  # vuln-code-snippet vuln-line infodisclosure_df_to_webroot
}
# vuln-code-snippet end infodisclosure_df_to_webroot

# vuln-code-snippet start infodisclosure_auth_header_log
debug_api_call() {
    local auth_token="$1"
    # The grep extracts the Authorization header line and tee writes it to a
    # log file. The bearer token is deliberately captured and persisted.
    # /tmp/auth_debug.log is world-readable by default.
    curl -v "https://api.example.com/v1/whoami" \
        -H "Authorization: Bearer ${auth_token}" 2>&1 \
        | grep -i authorization \
        | tee /tmp/auth_debug.log  # vuln-code-snippet vuln-line infodisclosure_auth_header_log
}
# vuln-code-snippet end infodisclosure_auth_header_log

# vuln-code-snippet start infodisclosure_aws_identity_log
log_aws_context() {
    # get-caller-identity returns the AWS Account ID, IAM User/Role ARN, and
    # User ID. These identifiers aid attackers in privilege escalation and
    # lateral movement. Logging them persistently to /var/log/ makes them
    # available to any process that can read the log.
    aws sts get-caller-identity | tee /var/log/aws_context.log  # vuln-code-snippet vuln-line infodisclosure_aws_identity_log
}
# vuln-code-snippet end infodisclosure_aws_identity_log

# vuln-code-snippet start infodisclosure_mysqldump_to_tmp
backup_database() {
    local db_user="$1"
    local db_pass="$2"
    local db_name="$3"
    # The password appears as a mysqldump command-line argument (-p"$db_pass"),
    # visible in ps aux during execution. The dump file is written to /tmp with
    # a predictable timestamp name, world-readable by default, containing the
    # full database schema and data.
    mysqldump -u"$db_user" -p"$db_pass" "$db_name" > "/tmp/dump_$(date +%s).sql"  # vuln-code-snippet vuln-line infodisclosure_mysqldump_to_tmp
}
# vuln-code-snippet end infodisclosure_mysqldump_to_tmp

# vuln-code-snippet start infodisclosure_incomplete_redaction
log_with_redaction() {
    # grep -v PASSWORD removes only lines containing the exact string
    # "PASSWORD". Variables named PASS, SECRET, KEY, TOKEN, API_KEY, DB_CRED
    # — and any variable where the assignment is on a line not containing
    # "PASSWORD" — are still logged. Incomplete pattern-based redaction
    # provides false security.
    { set -x; source /etc/app/config.sh; set +x; } 2>&1 |
        grep -v PASSWORD |  # vuln-code-snippet vuln-line infodisclosure_incomplete_redaction
        tee /var/log/config_load.log
}
# vuln-code-snippet end infodisclosure_incomplete_redaction

# vuln-code-snippet start infodisclosure_shadow_error_msg
validate_system_user() {
    local username="$1"
    # getent shadow reads the /etc/shadow file (password hashes). The hash
    # value is echoed to stdout as part of the validation result message. Even
    # password hashes should not be disclosed — they enable offline cracking
    # attacks.
    local entry
    entry=$(getent shadow "$username" 2>&1)
    echo "Validation result for ${username}: ${entry}"  # vuln-code-snippet vuln-line infodisclosure_shadow_error_msg
}
# vuln-code-snippet end infodisclosure_shadow_error_msg

# vuln-code-snippet start infodisclosure_strace_log
trace_application() {
    local app_cmd="$1"
    # strace with trace=read,write captures all read() and write() syscall
    # arguments including file contents and network data. This includes any
    # credentials, tokens, or sensitive data the application reads or writes
    # during execution. The trace output persists in /var/log/.
    strace -o /var/log/app_trace.log -e trace=read,write "$app_cmd"  # vuln-code-snippet vuln-line infodisclosure_strace_log
}
# vuln-code-snippet end infodisclosure_strace_log

# --- Safe variants ---

# vuln-code-snippet start infodisclosure_host_truncated
log_deployment_target() {
    local full_hostname="$1"
    # ${full_hostname%%.*} truncates to the first label (e.g., db-prod-01 from
    # db-prod-01.internal.corp). Only the hostname label is logged, not
    # credentials, IP addresses, or domain structure.
    local short_host="${full_hostname%%.*}"
    echo "Deployment target: ${short_host}"  # vuln-code-snippet safe-line infodisclosure_host_truncated
}
# vuln-code-snippet end infodisclosure_host_truncated

# vuln-code-snippet start infodisclosure_password_redacted
run_db_command_logged() {
    local db_pass="$1"
    local query="$2"
    # The sed expression replaces the -pPASSWORD argument with [REDACTED]
    # before tee writes to the log. The actual password value is stripped from
    # all logged output.
    mysql -h db.internal.corp -uapp -p"$db_pass" appdb -e "$query" 2>&1 |
        sed 's/-p[^ ]*/[REDACTED]/g' |  # vuln-code-snippet safe-line infodisclosure_password_redacted
        tee /var/log/db_commands.log
}
# vuln-code-snippet end infodisclosure_password_redacted

# vuln-code-snippet start infodisclosure_exit_code_only
report_exit_status() {
    local exit_code="$1"
    # Only the numeric exit code is logged. No credentials, paths, or sensitive
    # system information are revealed.
    echo "Process exited with code: ${exit_code}"  # vuln-code-snippet safe-line infodisclosure_exit_code_only
}
# vuln-code-snippet end infodisclosure_exit_code_only

# vuln-code-snippet start infodisclosure_resource_id_log
log_resource_access() {
    local resource_id="$1"
    # Only the resource ID (a non-sensitive identifier) is logged. No
    # credentials, user data, or system internals are included in the log entry.
    logger -t app "Request processed for resource: ${resource_id}"  # vuln-code-snippet safe-line infodisclosure_resource_id_log
}
# vuln-code-snippet end infodisclosure_resource_id_log

# vuln-code-snippet start infodisclosure_set_plus_x_wrap
run_sensitive_operation() {
    local secret="$1"
    # set +x explicitly disables bash execution tracing for the subshell block.
    # 2>/dev/null discards any trace output that might leak. The sensitive
    # operation runs without tracing.
    { set +x; process_secret "$secret"; } 2>/dev/null  # vuln-code-snippet safe-line infodisclosure_set_plus_x_wrap
}
# vuln-code-snippet end infodisclosure_set_plus_x_wrap

# vuln-code-snippet start infodisclosure_truncated_input
log_user_request() {
    local user_input="$1"
    # User input is truncated to 50 characters before logging. This bounds the
    # amount of data logged and prevents log injection via long crafted strings,
    # while ensuring no sensitive data beyond the first 50 chars is retained.
    local safe_msg
    safe_msg=$(echo "$user_input" | cut -c1-50)
    logger -t app "Request data: ${safe_msg}"  # vuln-code-snippet safe-line infodisclosure_truncated_input
}
# vuln-code-snippet end infodisclosure_truncated_input

# vuln-code-snippet start infodisclosure_timestamp_only
log_deploy_start() {
    # Only a timestamp and a hardcoded message string are logged. No user
    # input, credentials, environment variables, or system state are captured.
    echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] Deployment pipeline started"  # vuln-code-snippet safe-line infodisclosure_timestamp_only
}
# vuln-code-snippet end infodisclosure_timestamp_only

# vuln-code-snippet start infodisclosure_generic_error
handle_auth_failure() {
    # The error message is generic and reveals no detail about why
    # authentication failed, what credentials were checked, or what the
    # expected value is. This is the correct pattern for authentication
    # failure messages.
    echo "Authentication failed. Please check your credentials." >&2  # vuln-code-snippet safe-line infodisclosure_generic_error
}
# vuln-code-snippet end infodisclosure_generic_error

# vuln-code-snippet start infodisclosure_filtered_ps
report_app_processes() {
    # awk prints only columns 1 (user), 2 (PID), and 11 (command basename).
    # Command-line arguments (columns 12+) including any credential arguments
    # are excluded. The grep further limits to specific system users.
    ps aux | awk '{print $1, $2, $11}' | grep -E '^(app|www-data|nobody)'  # vuln-code-snippet safe-line infodisclosure_filtered_ps
}
# vuln-code-snippet end infodisclosure_filtered_ps

# vuln-code-snippet start infodisclosure_env_filtered_log
log_safe_environment() {
    # The grep -vE pattern excludes variables whose names contain common
    # secret-related terms before writing to the log. This is a reasonable
    # defense-in-depth pattern, though not exhaustive.
    env | grep -vE '(PASSWORD|SECRET|TOKEN|KEY|PASS|CRED|AUTH)' > /var/log/safe_env.log  # vuln-code-snippet safe-line infodisclosure_env_filtered_log
}
# vuln-code-snippet end infodisclosure_env_filtered_log

# vuln-code-snippet start infodisclosure_hostname_only
display_node_identity() {
    # Discrimination test: /etc/hostname contains only the system's hostname,
    # which is not sensitive. Tools that flag all file reads of /etc/* files
    # without distinguishing sensitive (/etc/shadow, /etc/passwd) from
    # non-sensitive (/etc/hostname, /etc/timezone) will FP here.
    cat /etc/hostname  # vuln-code-snippet safe-line infodisclosure_hostname_only
}
# vuln-code-snippet end infodisclosure_hostname_only

# vuln-code-snippet start infodisclosure_disk_usage_safe
check_data_partition() {
    # Discrimination test: checking disk usage on a specific data partition
    # reveals only size/usage metrics, not credentials or sensitive content.
    # The path /var/app/data is hardcoded.
    df -h /var/app/data  # vuln-code-snippet safe-line infodisclosure_disk_usage_safe
}
# vuln-code-snippet end infodisclosure_disk_usage_safe

# vuln-code-snippet start infodisclosure_status_code_only
check_api_health() {
    # -o /dev/null discards the response body; -w "%{http_code}" captures only
    # the numeric HTTP status code. No response headers, body content, or
    # credentials are logged.
    local http_code
    http_code=$(curl -s -o /dev/null -w "%{http_code}" "https://api.example.com/health")  # vuln-code-snippet safe-line infodisclosure_status_code_only
    echo "API health check: HTTP ${http_code}"
}
# vuln-code-snippet end infodisclosure_status_code_only

# vuln-code-snippet start infodisclosure_trace_to_deleted_tmp
trace_sensitive_op() {
    # Trace output is redirected to a mktemp file (unique, non-predictable
    # path) and the trap ensures deletion when the function exits. The trace
    # never reaches a persistent log file or shared location.
    local TMPLOG
    TMPLOG=$(mktemp)
    trap 'rm -f "$TMPLOG"' EXIT
    { set -x; run_sensitive_command; set +x; } 2>"$TMPLOG"  # vuln-code-snippet safe-line infodisclosure_trace_to_deleted_tmp
}
# vuln-code-snippet end infodisclosure_trace_to_deleted_tmp

# vuln-code-snippet start infodisclosure_build_id_only
log_build_metadata() {
    local build_id="$1"
    # Discrimination test: set -x tracing is active, but the only variable in
    # scope is build_id (a non-sensitive CI build identifier). No credentials,
    # tokens, or sensitive system data are in scope during the traced section.
    set -x
    echo "Starting build pipeline for build ID: ${build_id}"  # vuln-code-snippet safe-line infodisclosure_build_id_only
    set +x
}
# vuln-code-snippet end infodisclosure_build_id_only
