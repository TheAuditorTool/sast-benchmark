#!/bin/bash
# Log Injection Extended Test Cases (CWE-117)
# Additional patterns beyond v0.4.0: journald injection, GELF, CRLF injection,
# syslog format breaking, User-Agent logging, eval in log path, CSV log injection.

# vuln-code-snippet start loginjection_journald_tag
log_user_event() {
    local user_tag="$1"
    local user_msg="$2"
    # systemd-cat sends the message to the journal. user_tag is used as the
    # SYSLOG_IDENTIFIER field. Injecting ANSI escape sequences or control characters
    # into user_tag corrupts journald output when rendered in terminals. Newlines in
    # user_tag create additional journal entries with attacker-controlled content.
    systemd-cat -t "$user_tag" <<< "$user_msg"  # vuln-code-snippet vuln-line loginjection_journald_tag
}
# vuln-code-snippet end loginjection_journald_tag

# vuln-code-snippet start loginjection_gelf_json_inject
send_gelf_log() {
    local user_msg="$1"
    # user_msg is interpolated directly into a JSON string. Injecting a double-quote
    # terminates the JSON string field and allows injection of additional GELF fields:
    # supplying ","level":0,"_injected":"true breaks out of short_message and adds
    # attacker-controlled fields to the log entry.
    curl -s -X POST "http://graylog.internal:12201/gelf" \
        -H "Content-Type: application/json" \
        -d "{\"version\":\"1.1\",\"host\":\"$(hostname)\",\"short_message\":\"${user_msg}\"}"  # vuln-code-snippet vuln-line loginjection_gelf_json_inject
}
# vuln-code-snippet end loginjection_gelf_json_inject

# vuln-code-snippet start loginjection_crlf_log_append
append_access_log() {
    local level="$1"
    local msg="$2"
    # Neither level nor msg is sanitized for CRLF sequences (\r\n). Injecting
    # \n2024-01-01 00:00:00 [INFO] Legitimate looking entry into msg creates a
    # synthetic log line that appears legitimate, enabling log forgery.
    echo "[${level}] ${msg}" >> /var/log/app.log  # vuln-code-snippet vuln-line loginjection_crlf_log_append
}
# vuln-code-snippet end loginjection_crlf_log_append

# vuln-code-snippet start loginjection_logger_username
log_user_login() {
    local username="$1"
    # Syslog messages containing newlines split into multiple syslog entries. A username
    # containing \n followed by fake syslog priority markers (e.g.,
    # \nFeb 1 00:00:00 server sudo: root: command not found) injects synthetic entries
    # into the system log, potentially framing other users or obscuring attacker activity.
    logger -p local0.info "User ${username} logged in successfully"  # vuln-code-snippet vuln-line loginjection_logger_username
}
# vuln-code-snippet end loginjection_logger_username

# vuln-code-snippet start loginjection_awk_begin_log
audit_log_entry() {
    local username="$1"
    local action="$2"
    # awk -v assigns variables but does not escape them for awk injection. A username
    # containing a single-quote or awk control characters (;, {, }) can break the awk
    # expression context. Additionally, newlines in the variable values produce multiple
    # log lines.
    awk -v user="$username" -v act="$action" \
        'BEGIN{print strftime("%Y-%m-%dT%H:%M:%S"), user, act}' >> /var/log/audit.log  # vuln-code-snippet vuln-line loginjection_awk_begin_log
}
# vuln-code-snippet end loginjection_awk_begin_log

# vuln-code-snippet start loginjection_user_report_header
generate_log_report() {
    local report_user="$1"
    # report_user is interpolated into the report header. Injecting newlines allows the
    # attacker to add fabricated log entries to the report. If report_user contains the
    # mail separator sequence (-- or subject headers), it can manipulate the email
    # structure.
    { echo "=== Report for: ${report_user} ==="; cat /var/log/app.log; } | mail -s "Log Report" admin@example.com  # vuln-code-snippet vuln-line loginjection_user_report_header
}
# vuln-code-snippet end loginjection_user_report_header

# vuln-code-snippet start loginjection_python_format
log_via_python() {
    local user_input="$1"
    # user_input is interpolated by bash into the Python string before Python even parses
    # it. Injecting ') or die\nlogging.critical(' breaks out of the string literal context
    # in the Python code, executing injected Python expressions.
    python3 -c "import logging; logging.warning('User action: ${user_input}')"  # vuln-code-snippet vuln-line loginjection_python_format
}
# vuln-code-snippet end loginjection_python_format

# vuln-code-snippet start loginjection_tee_heredoc
log_event_with_tee() {
    local user_input="$1"
    # user_input is interpolated in the here-string before tee writes it to the log.
    # Newlines in user_input create additional log lines with attacker-controlled content.
    # ANSI escape sequences can corrupt log rendering.
    tee -a /var/log/app.log <<< "$(date +%Y-%m-%dT%H:%M:%S) ${user_input}"  # vuln-code-snippet vuln-line loginjection_tee_heredoc
}
# vuln-code-snippet end loginjection_tee_heredoc

# vuln-code-snippet start loginjection_user_agent_log
log_http_request() {
    local user_agent="$1"
    # HTTP User-Agent headers are entirely attacker-controlled. Newline injection creates
    # synthetic log entries. This is a classic and well-documented log injection vector:
    # crafted User-Agent strings like 200 - - [01/Jan/2024]\nGET /admin HTTP/1.1 can
    # create false access log entries.
    echo "$user_agent" >> /var/log/access.log  # vuln-code-snippet vuln-line loginjection_user_agent_log
}
# vuln-code-snippet end loginjection_user_agent_log

# vuln-code-snippet start loginjection_json_manual_build
write_json_log() {
    local user_input="$1"
    # Injecting a double-quote into user_input breaks the JSON string: supplying
    # ","injected":true,"level":"error" produces a valid JSON object with extra
    # attacker-controlled fields. Subsequent log parsers (Elasticsearch, Splunk) ingest
    # the injected fields as legitimate log data.
    echo "{\"timestamp\":\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",\"message\":\"${user_input}\"}" >> app.log  # vuln-code-snippet vuln-line loginjection_json_manual_build
}
# vuln-code-snippet end loginjection_json_manual_build

# vuln-code-snippet start loginjection_udp_syslog
send_remote_syslog() {
    local error_msg="$1"
    # UDP syslog (RFC 3164) has no authentication. Injecting newlines into error_msg
    # produces multiple syslog packets. The remote syslog server receives these as
    # separate log entries with attacker-controlled priorities, hostnames, and messages.
    echo "$(date) ERROR: ${error_msg}" | nc -u syslog.corp.internal 514  # vuln-code-snippet vuln-line loginjection_udp_syslog
}
# vuln-code-snippet end loginjection_udp_syslog

# vuln-code-snippet start loginjection_eval_in_log_path
dynamic_log_write() {
    local user_action="$1"
    # user_action is expanded by bash before eval processes it. Injecting ; rm -rf /var
    # or $(malicious_command) executes arbitrary commands. The eval context makes this
    # a code injection vulnerability in addition to log injection — the logging mechanism
    # itself becomes an RCE vector.
    eval "echo $(date): ${user_action} >> /var/log/app.log"  # vuln-code-snippet vuln-line loginjection_eval_in_log_path
}
# vuln-code-snippet end loginjection_eval_in_log_path

# vuln-code-snippet start loginjection_log_cmd_variable
flexible_logger() {
    local user_msg="$1"
    # LOG_CMD is an environment variable that could be set to any command (echo, logger,
    # tee, or an attacker-controlled value if $LOG_CMD is user-influenced). Even if
    # user_msg is quoted, a malicious LOG_CMD value (e.g., LOG_CMD="rm -rf /tmp;logger")
    # executes arbitrary commands.
    ${LOG_CMD} "$user_msg"  # vuln-code-snippet vuln-line loginjection_log_cmd_variable
}
# vuln-code-snippet end loginjection_log_cmd_variable

# vuln-code-snippet start loginjection_sed_insert
insert_log_entry() {
    local date_marker="$1"
    local user_input="$2"
    # user_input is embedded in a sed address command. A newline in user_input creates
    # additional sed commands. Injecting \n/a\malicious_content adds attacker-controlled
    # content at an attacker-chosen insertion point. Backslash sequences in user_input
    # may also trigger sed escaping behaviors.
    sed -i "/${date_marker}/a\\${user_input}" /var/log/structured.log  # vuln-code-snippet vuln-line loginjection_sed_insert
}
# vuln-code-snippet end loginjection_sed_insert

# vuln-code-snippet start loginjection_csv_raw_append
audit_csv_log() {
    local timestamp="$1"
    local user="$2"
    local action="$3"
    # CSV fields are not escaped. A user value containing a comma splits into additional
    # columns. A user value containing a newline creates a new row. A user value
    # containing double-quotes breaks RFC 4180 CSV quoting. Log parsers and SIEMs that
    # ingest this CSV file will misparse the injected entries.
    printf "%s,%s,%s\n" "$timestamp" "$user" "$action" >> /var/log/audit.csv  # vuln-code-snippet vuln-line loginjection_csv_raw_append
}
# vuln-code-snippet end loginjection_csv_raw_append

# --- Safe variants ---

# vuln-code-snippet start loginjection_ip_digits_only
log_client_connection() {
    local ip="$1"
    # tr -dc '0-9.' deletes all characters except digits and dots. Any shell
    # metacharacters, newlines, control sequences, or ANSI escapes are stripped before
    # logging. The sanitized IP can only contain valid IPv4 characters.
    sanitized_ip=$(printf '%s' "$ip" | tr -dc '0-9.')
    logger -p auth.info -- "Connection from ${sanitized_ip}"  # vuln-code-snippet safe-line loginjection_ip_digits_only
}
# vuln-code-snippet end loginjection_ip_digits_only

# vuln-code-snippet start loginjection_printf_q_escape
log_user_message() {
    local user_msg="$1"
    # printf %q escapes all shell metacharacters and special characters in user_msg. The
    # resulting string is safe to embed in shell-parsed contexts and cannot contain
    # unescaped newlines or control characters that would inject log entries.
    sanitized=$(printf '%q' "$user_msg")
    echo "$(date +%Y-%m-%dT%H:%M:%S) ${sanitized}" >> /var/log/app.log  # vuln-code-snippet safe-line loginjection_printf_q_escape
}
# vuln-code-snippet end loginjection_printf_q_escape

# vuln-code-snippet start loginjection_jq_json_safe
write_structured_log() {
    local user_input="$1"
    # jq --arg passes user_input as a jq string variable. jq handles all JSON string
    # escaping internally — double quotes, backslashes, and control characters in
    # user_input are properly escaped in the output JSON. No injection into JSON
    # structure is possible.
    jq -n --arg msg "$user_input" --arg ts "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
        '{"timestamp": $ts, "message": $msg}' >> structured.log  # vuln-code-snippet safe-line loginjection_jq_json_safe
}
# vuln-code-snippet end loginjection_jq_json_safe

# vuln-code-snippet start loginjection_param_strip
log_action_name() {
    local action="$1"
    # The parameter expansion ${action//[^a-zA-Z0-9_]/} strips all characters that are
    # not alphanumeric or underscore. Newlines, control characters, semicolons, and all
    # other injection vectors are removed before the value reaches the log.
    echo "Action: ${action//[^a-zA-Z0-9_]/}" >> /var/log/app.log  # vuln-code-snippet safe-line loginjection_param_strip
}
# vuln-code-snippet end loginjection_param_strip

# vuln-code-snippet start loginjection_python_json_dumps
structured_python_log() {
    local user_msg="$1"
    # user_msg is passed as a separate argument (sys.argv[1]) rather than interpolated
    # into the Python source code. json.dumps() escapes all special characters in msg.
    # No shell injection or JSON injection is possible.
    python3 -c "
import json, sys, datetime
entry = {'ts': datetime.datetime.utcnow().isoformat(), 'msg': sys.argv[1]}
print(json.dumps(entry))
" "$user_msg" >> app.log  # vuln-code-snippet safe-line loginjection_python_json_dumps
}
# vuln-code-snippet end loginjection_python_json_dumps

# vuln-code-snippet start loginjection_static_message
log_build_complete() {
    # The message string is entirely hardcoded. No user input is incorporated. This is a
    # discrimination test for tools that flag all printf/echo calls to log files without
    # checking whether user input is involved.
    printf '%s\n' "$(date +%Y-%m-%dT%H:%M:%S) Build pipeline completed successfully"  # vuln-code-snippet safe-line loginjection_static_message
}
# vuln-code-snippet end loginjection_static_message

# vuln-code-snippet start loginjection_build_num_only
log_build_number() {
    local build_number="$1"
    # build_number is validated to contain only digits before use in the log message.
    # Digits cannot contain newlines, control characters, or any injection vectors.
    if [[ ! "$build_number" =~ ^[0-9]+$ ]]; then
        echo "Invalid build number" >&2; return 1
    fi
    echo "Build ${build_number} started" >> /var/log/build.log  # vuln-code-snippet safe-line loginjection_build_num_only
}
# vuln-code-snippet end loginjection_build_num_only

# vuln-code-snippet start loginjection_control_strip
sanitize_and_log() {
    local user_msg="$1"
    sanitize_log() {
        printf '%s' "$1" | tr -d '\n\r\033'
    }
    # tr -d '\n\r\033' strips carriage returns (CRLF injection), newlines (log line
    # injection), and ESC characters (ANSI escape injection). The sanitized output
    # cannot create new log lines or terminal control sequences.
    sanitize_log "$user_msg" >> /var/log/app.log  # vuln-code-snippet safe-line loginjection_control_strip
}
# vuln-code-snippet end loginjection_control_strip

# vuln-code-snippet start loginjection_validated_email_log
log_user_access() {
    local user_email="$1"
    # The email regex permits only alphanumerics, dots, underscores, percent, plus,
    # hyphen, and the @ symbol. None of these can produce newlines, control characters,
    # or log injection payloads.
    if [[ ! "$user_email" =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$ ]]; then
        echo "Invalid email" >&2; return 1
    fi
    echo "$user_email" >> /var/log/access.log  # vuln-code-snippet safe-line loginjection_validated_email_log
}
# vuln-code-snippet end loginjection_validated_email_log

# vuln-code-snippet start loginjection_systemd_escape_name
log_unit_event() {
    local user_unit_name="$1"
    # systemd-escape converts arbitrary strings to systemd-safe unit name format,
    # replacing special characters with \xHH escape sequences. The result cannot contain
    # newlines, spaces, or shell metacharacters.
    escaped_name=$(systemd-escape "$user_unit_name")
    echo "Unit ${escaped_name} started" >> /var/log/units.log  # vuln-code-snippet safe-line loginjection_systemd_escape_name
}
# vuln-code-snippet end loginjection_systemd_escape_name

# vuln-code-snippet start loginjection_syslog_drop_nl
log_connection_source() {
    local ip="$1"
    # ${ip//[^0-9.]/} strips everything except digits and dots from the IP address
    # string before it reaches the logger command. No injection characters can survive
    # this filter.
    clean_ip="${ip//[^0-9.]/}"
    logger -e "Connection from ${clean_ip}"  # vuln-code-snippet safe-line loginjection_syslog_drop_nl
}
# vuln-code-snippet end loginjection_syslog_drop_nl

# vuln-code-snippet start loginjection_no_user_input
log_deploy_event() {
    # No user input is involved. The log entry consists only of a system timestamp and a
    # hardcoded string. This is a discrimination test for tools that flag all
    # log-appending operations regardless of taint status.
    { date -u +%Y-%m-%dT%H:%M:%SZ; echo "Deployment completed"; } >> /var/log/deploy.log  # vuln-code-snippet safe-line loginjection_no_user_input
}
# vuln-code-snippet end loginjection_no_user_input

# vuln-code-snippet start loginjection_numeric_count_only
log_record_count() {
    local count="$1"
    # count is validated as a non-negative integer. Integer strings cannot contain
    # injection characters. The log message has no injection surface.
    if [[ ! "$count" =~ ^[0-9]+$ ]]; then
        echo "Invalid count" >&2; return 1
    fi
    echo "Processed ${count} records"  # vuln-code-snippet safe-line loginjection_numeric_count_only
}
# vuln-code-snippet end loginjection_numeric_count_only

# vuln-code-snippet start loginjection_base64_user_data
log_encoded_payload() {
    local user_input="$1"
    # base64 encoding converts arbitrary binary input to a character set of
    # [A-Za-z0-9+/=]. The encoded output cannot contain newlines, control characters,
    # or any injection vectors. Log parsers can decode the payload safely when needed.
    encoded=$(printf '%s' "$user_input" | base64 -w0)
    echo "$(date) payload=${encoded}" >> app.log  # vuln-code-snippet safe-line loginjection_base64_user_data
}
# vuln-code-snippet end loginjection_base64_user_data

# vuln-code-snippet start loginjection_json_printf_structured
write_formatted_log() {
    local level="$1"
    local msg="$2"
    # tr -d '"\' strips double-quotes and backslashes before msg is embedded in JSON.
    # The printf format string is a literal (not user-controlled). The resulting JSON
    # has no injection surface for double-quote termination or backslash escape attacks.
    clean_msg=$(printf '%s' "$msg" | tr -d $'"\\\\')
    printf '{"ts":"%s","level":"%s","msg":"%s"}\n' \
        "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
        "${level^^}" \
        "$clean_msg" >> app.log  # vuln-code-snippet safe-line loginjection_json_printf_structured
}
# vuln-code-snippet end loginjection_json_printf_structured
