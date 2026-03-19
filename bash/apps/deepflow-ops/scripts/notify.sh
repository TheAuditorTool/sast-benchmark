#!/bin/bash
# notify.sh - Notification script for DeployBot
#
# CONTAINS INTENTIONAL VULNERABILITIES FOR TAINT ANALYSIS
#
# Usage: ./notify.sh <message> [webhook_url]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/common.sh"
source "$SCRIPT_DIR/../lib/validate.sh"

# Arguments from Python subprocess
MESSAGE="$1"
WEBHOOK_URL="${2:-$WEBHOOK_URL}"
SLACK_WEBHOOK="${SLACK_WEBHOOK:-}"
EMAIL_TO="${EMAIL_TO:-}"

log_info "Sending notification"
log_info "Message: $MESSAGE"

# vuln-code-snippet start dfo_notify_safe_encoded
if [[ "${SAFE_MODE:-false}" == "true" ]]; then
    # === SAFE PATH ===
    log_info "Running in SAFE MODE"

    # SAFE: Validate message content
    if ! validate_message "$MESSAGE"; then  # vuln-code-snippet safe-line dfo_notify_safe_encoded
        log_error "Message validation failed"
        exit 1
    fi

    # SAFE: Validate webhook URL
    if [[ -n "$WEBHOOK_URL" ]]; then
        if ! validate_url "$WEBHOOK_URL"; then
            log_error "Webhook URL validation failed"
            exit 1
        fi
    fi

    # SAFE: Use --data-urlencode for curl
    if [[ -n "$WEBHOOK_URL" ]]; then
        curl -s -X POST "$WEBHOOK_URL" \
            --data-urlencode "message=$MESSAGE" \
            -H "Content-Type: application/x-www-form-urlencoded"
        log_info "Notification sent to webhook"
    fi

    # SAFE: Use printf %q for JSON construction
    if [[ -n "$SLACK_WEBHOOK" ]]; then
        SAFE_MESSAGE=$(printf '%s' "$MESSAGE" | jq -Rs .)
        curl -s -X POST "$SLACK_WEBHOOK" \
            -H "Content-Type: application/json" \
            -d "{\"text\": $SAFE_MESSAGE}"
        log_info "Notification sent to Slack"
    fi
# vuln-code-snippet end dfo_notify_safe_encoded

else
    # === VULNERABLE PATH ===
    log_warn "Running in UNSAFE MODE"

    # VULNERABLE: User message directly in curl data
    # TAINT FLOW: $1 -> curl -d (SSRF / Data Exfiltration)
    if [[ -n "$WEBHOOK_URL" ]]; then
        log_info "Sending to webhook: $WEBHOOK_URL"
        curl -s -X POST "$WEBHOOK_URL" -d "message=$MESSAGE"
    fi

    # VULNERABLE: User message in JSON without escaping
    # TAINT FLOW: $MESSAGE -> curl body (Injection)
    # vuln-code-snippet start dfo_notify_json_injection
    if [[ -n "$SLACK_WEBHOOK" ]]; then
        log_info "Sending to Slack"
        curl -s -X POST "$SLACK_WEBHOOK" \
            -H "Content-Type: application/json" \
            -d "{\"text\": \"$MESSAGE\"}"  # vuln-code-snippet vuln-line dfo_notify_json_injection
    fi
    # vuln-code-snippet end dfo_notify_json_injection

    # VULNERABLE: User-controlled URL in curl
    # TAINT FLOW: $CUSTOM_WEBHOOK -> curl URL (SSRF)
    if [[ -n "$CUSTOM_WEBHOOK" ]]; then
        log_info "Sending to custom webhook: $CUSTOM_WEBHOOK"
        curl -s -X POST "$CUSTOM_WEBHOOK" -d "$MESSAGE"
    fi

    # VULNERABLE: Send email with user content
    # TAINT FLOW: $MESSAGE -> mail command (Command Injection)
    # vuln-code-snippet start dfo_notify_mail_unquoted
    if [[ -n "$EMAIL_TO" ]]; then
        log_info "Sending email to: $EMAIL_TO"
        echo "$MESSAGE" | mail -s "DeployBot Notification" $EMAIL_TO  # vuln-code-snippet vuln-line dfo_notify_mail_unquoted
    fi
    # vuln-code-snippet end dfo_notify_mail_unquoted

    # VULNERABLE: Execute notification callback
    # TAINT FLOW: $NOTIFY_CALLBACK -> eval (Command Injection)
    # vuln-code-snippet start dfo_notify_eval_callback
    if [[ -n "$NOTIFY_CALLBACK" ]]; then
        log_info "Executing notification callback"
        eval "$NOTIFY_CALLBACK"  # vuln-code-snippet vuln-line dfo_notify_eval_callback
    fi
    # vuln-code-snippet end dfo_notify_eval_callback
fi

log_info "Notification completed"
exit 0
