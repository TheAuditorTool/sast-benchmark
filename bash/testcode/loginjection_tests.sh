#!/bin/bash
# Log Injection Test Cases (CWE-117)
# User input written to logs without neutralizing control characters.
# Attackers inject newlines/CRLF to forge log entries, hide traces, or
# exploit downstream log parsers (SIEM injection, log4shell-style).

# vuln-code-snippet start loginjection_echo_raw
log_user_action() {
    local username="$1"
    local action="$2"
    # User-controlled $action written directly — attacker injects
    # \n to forge a new log line (e.g., "login\n2026-04-07 admin granted root").
    echo "$(date +%F) ${username}: ${action}" >> /var/log/app.log  # vuln-code-snippet vuln-line loginjection_echo_raw
}
# vuln-code-snippet end loginjection_echo_raw

# vuln-code-snippet start loginjection_printf_unescaped
log_request() {
    local method="$1"
    local path="$2"
    # printf with %s passes control chars verbatim — \r\n in $path
    # lets attacker inject fabricated log entries.
    printf "%s %s %s\n" "$(date +%T)" "$method" "$path" >> /var/log/access.log  # vuln-code-snippet vuln-line loginjection_printf_unescaped
}
# vuln-code-snippet end loginjection_printf_unescaped

# vuln-code-snippet start loginjection_logger_direct
log_via_syslog() {
    local user_input="$1"
    # logger sends to syslog — injected newlines create separate syslog
    # entries, each with a valid-looking timestamp from syslogd.
    logger -t myapp "User query: ${user_input}"  # vuln-code-snippet vuln-line loginjection_logger_direct
}
# vuln-code-snippet end loginjection_logger_direct

# vuln-code-snippet start loginjection_append_redirect
write_audit_entry() {
    local event_data="$1"
    # Raw append — no sanitization of control characters.
    echo "[AUDIT] ${event_data}" >> /var/log/audit.log  # vuln-code-snippet vuln-line loginjection_append_redirect
}
# vuln-code-snippet end loginjection_append_redirect

# vuln-code-snippet start loginjection_tee_passthrough
log_and_display() {
    local message="$1"
    # tee passes raw bytes — ANSI escape sequences in $message can
    # overwrite terminal output AND corrupt the log file.
    echo "$(date +%F) ${message}" | tee -a /var/log/app.log  # vuln-code-snippet vuln-line loginjection_tee_passthrough
}
# vuln-code-snippet end loginjection_tee_passthrough

# vuln-code-snippet start loginjection_error_log_raw
log_failed_login() {
    local ip="$1"
    local supplied_user="$2"
    # Attacker controls $supplied_user — injects newline + fake "successful
    # login" entry to hide brute-force attempts in log review.
    echo "$(date +%FT%T) FAILED LOGIN from ${ip} user=${supplied_user}" >> /var/log/auth.log  # vuln-code-snippet vuln-line loginjection_error_log_raw
}
# vuln-code-snippet end loginjection_error_log_raw

# vuln-code-snippet start loginjection_csv_log_unescaped
log_to_csv() {
    local user="$1"
    local query="$2"
    # $query may contain commas, newlines, or quotes — breaks CSV structure
    # and injects new rows when parsed by downstream analytics.
    echo "$(date +%s),${user},${query}" >> /var/log/queries.csv  # vuln-code-snippet vuln-line loginjection_csv_log_unescaped
}
# vuln-code-snippet end loginjection_csv_log_unescaped

# vuln-code-snippet start loginjection_heredoc_log
write_detailed_log() {
    local request_body="$1"
    # Heredoc expands $request_body — embedded control chars flow into
    # the log file unmodified.
    cat >> /var/log/detailed.log << EOF
[$(date +%FT%T)] Request body: ${request_body}
EOF
    # vuln-code-snippet vuln-line loginjection_heredoc_log
}
# vuln-code-snippet end loginjection_heredoc_log

# vuln-code-snippet start loginjection_format_string_log
log_custom_format() {
    local template="$1"
    local value="$2"
    # $template is user-controlled format string — attacker supplies
    # "%s\n[FAKE] admin logged in\n%s" to inject entries.
    printf "$template" "$value" >> /var/log/custom.log  # vuln-code-snippet vuln-line loginjection_format_string_log
}
# vuln-code-snippet end loginjection_format_string_log

# vuln-code-snippet start loginjection_multiline_concat
aggregate_log_entries() {
    local user_notes="$1"
    local log_line="[$(date +%FT%T)] notes=${user_notes}"
    # Concatenation preserves embedded newlines in $user_notes.
    echo "$log_line" >> /var/log/notes.log  # vuln-code-snippet vuln-line loginjection_multiline_concat
}
# vuln-code-snippet end loginjection_multiline_concat

# --- Safe variants ---

# vuln-code-snippet start loginjection_printf_q_escaped
log_user_action_safe() {
    local username="$1"
    local action="$2"
    # printf %q escapes all control characters (newlines, tabs, ANSI)
    # into their shell-quoted representations (\n, \t, etc.).
    local safe_action
    safe_action=$(printf '%q' "$action")
    echo "$(date +%F) ${username}: ${safe_action}" >> /var/log/app.log  # vuln-code-snippet safe-line loginjection_printf_q_escaped
}
# vuln-code-snippet end loginjection_printf_q_escaped

# vuln-code-snippet start loginjection_tr_sanitized
log_request_sanitized() {
    local method="$1"
    local path="$2"
    # tr -d strips carriage returns, newlines, and common control chars.
    local clean_path
    clean_path=$(echo "$path" | tr -d '\n\r\t\000-\037')
    echo "$(date +%T) ${method} ${clean_path}" >> /var/log/access.log  # vuln-code-snippet safe-line loginjection_tr_sanitized
}
# vuln-code-snippet end loginjection_tr_sanitized

# vuln-code-snippet start loginjection_allowlist_value
log_status_change() {
    local new_status="$1"
    # Only accept known status values — rejects anything with control chars.
    case "$new_status" in
        active|inactive|suspended|deleted)
            echo "$(date +%F) Status changed to: ${new_status}" >> /var/log/status.log  # vuln-code-snippet safe-line loginjection_allowlist_value
            ;;
        *)
            echo "Invalid status value" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end loginjection_allowlist_value

# vuln-code-snippet start loginjection_jq_structured
log_structured_json() {
    local username="$1"
    local action="$2"
    # jq --arg safely encodes control characters as JSON escape sequences.
    # Downstream log parsers (ELK, Splunk) handle JSON natively.
    jq -n --arg ts "$(date +%FT%T)" --arg user "$username" --arg act "$action" \
        '{timestamp: $ts, user: $user, action: $act}' >> /var/log/structured.jsonl  # vuln-code-snippet safe-line loginjection_jq_structured
}
# vuln-code-snippet end loginjection_jq_structured

# vuln-code-snippet start loginjection_hardcoded_message
log_startup_event() {
    # No user input — hardcoded message cannot be injected.
    echo "$(date +%FT%T) Application started successfully" >> /var/log/app.log  # vuln-code-snippet safe-line loginjection_hardcoded_message
}
# vuln-code-snippet end loginjection_hardcoded_message

# vuln-code-snippet start loginjection_sed_control_strip
log_cleaned_input() {
    local user_input="$1"
    # sed strips all ASCII control characters (0x00-0x1F, 0x7F).
    local cleaned
    cleaned=$(echo "$user_input" | sed 's/[[:cntrl:]]//g')
    echo "$(date +%F) Input: ${cleaned}" >> /var/log/app.log  # vuln-code-snippet safe-line loginjection_sed_control_strip
}
# vuln-code-snippet end loginjection_sed_control_strip

# vuln-code-snippet start loginjection_base64_encoded
log_binary_safe() {
    local raw_data="$1"
    # base64 encoding eliminates all control characters — safe for any
    # log parser. Decoder recovers original data when needed.
    local encoded
    encoded=$(echo -n "$raw_data" | base64)
    echo "$(date +%F) data_b64=${encoded}" >> /var/log/app.log  # vuln-code-snippet safe-line loginjection_base64_encoded
}
# vuln-code-snippet end loginjection_base64_encoded

# vuln-code-snippet start loginjection_numeric_only
log_request_id() {
    local req_id="$1"
    # Reject non-numeric input — integers cannot contain control chars.
    if [[ "$req_id" =~ ^[0-9]+$ ]]; then
        echo "$(date +%F) Processing request ${req_id}" >> /var/log/app.log  # vuln-code-snippet safe-line loginjection_numeric_only
    else
        echo "Invalid request ID" >&2
        return 1
    fi
}
# vuln-code-snippet end loginjection_numeric_only

# vuln-code-snippet start loginjection_escaped_newlines
log_with_escape() {
    local user_msg="$1"
    # Replace literal newlines with \n string representation.
    local escaped
    escaped="${user_msg//$'\n'/\\n}"
    escaped="${escaped//$'\r'/\\r}"
    echo "$(date +%F) msg=${escaped}" >> /var/log/app.log  # vuln-code-snippet safe-line loginjection_escaped_newlines
}
# vuln-code-snippet end loginjection_escaped_newlines

# vuln-code-snippet start loginjection_syslog_priority_safe
log_via_syslog_safe() {
    local user_input="$1"
    # logger --id logs PID; input is passed through printf %q to escape
    # control characters before reaching syslog.
    local safe_input
    safe_input=$(printf '%q' "$user_input")
    logger --id -t myapp "User query: ${safe_input}"  # vuln-code-snippet safe-line loginjection_syslog_priority_safe
}
# vuln-code-snippet end loginjection_syslog_priority_safe
