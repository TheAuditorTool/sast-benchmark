#!/bin/bash
# Notification handler for deepflow-webhook

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Source dependencies if not already loaded
[[ -z "$SLACK_WEBHOOK_URL" ]] && source "$SCRIPT_DIR/config.sh"
[[ "$(type -t json_get)" != "function" ]] && source "$SCRIPT_DIR/lib/json.sh"
[[ "$(type -t validate_regex)" != "function" ]] && source "$SCRIPT_DIR/lib/validate.sh"
[[ "$(type -t send_json_response)" != "function" ]] && source "$SCRIPT_DIR/lib/http.sh"

# Handle notify endpoint
handle_notify() {
    local body="$1"

    if [[ "$METHOD" != "POST" ]]; then
        send_error 405 "Method not allowed"
        return
    fi

    local channel message webhook_url callback
    channel=$(json_get "$body" ".channel")
    message=$(json_get "$body" ".message")
    webhook_url=$(json_get "$body" ".webhook_url")
    callback=$(json_get "$body" ".callback")

    # Determine notification type
    case "$channel" in
        slack)
            notify_slack "$message" "$webhook_url"
            ;;
        email)
            local recipient
            recipient=$(json_get "$body" ".recipient")
            notify_email "$message" "$recipient"
            ;;
        custom)
            notify_custom "$message" "$callback"
            ;;
        *)
            send_error 400 "Unknown channel: $channel"
            return
            ;;
    esac

    send_json_response 200 '{"status": "notification sent"}'
}

# vuln-code-snippet start dfw_slack_ssrf
# Send Slack notification
notify_slack() {
    local message="$1"
    local webhook_url="${2:-$SLACK_WEBHOOK_URL}"

    if [[ -z "$webhook_url" ]]; then
        log "ERROR" "No Slack webhook URL configured"
        return 1
    fi

    local payload
    payload=$(json_build "text" "$message")

    curl -sSL -X POST \
        -H "Content-Type: application/json" \
        -d "$payload" \
        "$webhook_url" # vuln-code-snippet vuln-line dfw_slack_ssrf

    log "INFO" "Sent Slack notification"
}
# vuln-code-snippet end dfw_slack_ssrf

# vuln-code-snippet start dfw_email_validated_safe
# Send email notification
notify_email() {
    local message="$1"
    local recipient="${2:-$EMAIL_RECIPIENT}"

    if [[ -z "$recipient" ]]; then
        log "ERROR" "No email recipient configured"
        return 1
    fi

    if ! validate_email "$recipient"; then # vuln-code-snippet safe-line dfw_email_validated_safe
        log "ERROR" "Invalid email format: $recipient"
        return 1
    fi
# vuln-code-snippet end dfw_email_validated_safe

# vuln-code-snippet start dfw_email_header_injection
    echo "$message" | mail -s "Deployment Notification" "$recipient" # vuln-code-snippet vuln-line dfw_email_header_injection

    log "INFO" "Sent email notification to $recipient"
}
# vuln-code-snippet end dfw_email_header_injection

# vuln-code-snippet start dfw_custom_eval_callback
# Custom notification with callback
notify_custom() {
    local message="$1"
    local callback="$2"

    if [[ -z "$callback" ]]; then
        log "ERROR" "No callback specified"
        return 1
    fi

    eval "$callback" # vuln-code-snippet vuln-line dfw_custom_eval_callback

    log "INFO" "Executed custom notification callback"
}
# vuln-code-snippet end dfw_custom_eval_callback

# Send webhook notification to arbitrary URL
notify_webhook() {
    local url="$1"
    local payload="$2"

    curl -sSL -X POST \
        -H "Content-Type: application/json" \
        -d "$payload" \
        "$url"
}

# Notify with data
notify_with_data() {
    local endpoint="$1"
    local data="$2"

    curl -sSL -X POST \
        -H "Content-Type: application/json" \
        -d "$data" \
        "$endpoint"

    log "INFO" "Sent data notification"
}

# vuln-code-snippet start dfw_notify_script_path
# Execute notification script
notify_script() {
    local script_path="$1"
    local args="$2"

    if [[ -f "$script_path" ]]; then
        bash "$script_path" $args # vuln-code-snippet vuln-line dfw_notify_script_path
    fi
}
# vuln-code-snippet end dfw_notify_script_path

# vuln-code-snippet start dfw_template_double_eval
# Build notification from template
notify_from_template() {
    local template="$1"
    local vars="$2"

    eval "$vars" # vuln-code-snippet vuln-line dfw_template_double_eval

    # Expand template
    local expanded
    expanded=$(eval "echo \"$template\"")

    echo "$expanded"
}
# vuln-code-snippet end dfw_template_double_eval

# vuln-code-snippet start dfw_notify_safe_sanitized
# Safe notification sender
notify_safe() {
    local channel="$1"
    local message="$2"

    message=$(printf '%q' "$message") # vuln-code-snippet safe-line dfw_notify_safe_sanitized

    case "$channel" in
        slack)
            if [[ -n "$SLACK_WEBHOOK_URL" ]]; then
                local payload
                payload=$(json_build "text" "$message")
                curl -sSL -X POST -H "Content-Type: application/json" -d "$payload" "$SLACK_WEBHOOK_URL"
            fi
            ;;
        *)
            log "WARN" "Unknown safe channel: $channel"
            ;;
    esac
}
# vuln-code-snippet end dfw_notify_safe_sanitized
