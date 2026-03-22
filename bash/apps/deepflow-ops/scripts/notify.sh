#!/bin/bash
# notify.sh - Notification script for DeployBot
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
    log_info "Running in SAFE MODE"
    if ! validate_message "$MESSAGE"; then  # vuln-code-snippet safe-line dfo_notify_safe_encoded
        log_error "Message validation failed"
        exit 1
    fi

    if [[ -n "$WEBHOOK_URL" ]]; then
        if ! validate_url "$WEBHOOK_URL"; then
            log_error "Webhook URL validation failed"
            exit 1
        fi
    fi

    # Use --data-urlencode for curl
    if [[ -n "$WEBHOOK_URL" ]]; then
        curl -s -X POST "$WEBHOOK_URL" \
            --data-urlencode "message=$MESSAGE" \
            -H "Content-Type: application/x-www-form-urlencoded"
        log_info "Notification sent to webhook"
    fi

    # Use jq for JSON construction
    if [[ -n "$SLACK_WEBHOOK" ]]; then
        SAFE_MESSAGE=$(printf '%s' "$MESSAGE" | jq -Rs .)
        curl -s -X POST "$SLACK_WEBHOOK" \
            -H "Content-Type: application/json" \
            -d "{\"text\": $SAFE_MESSAGE}"
        log_info "Notification sent to Slack"
    fi
# vuln-code-snippet end dfo_notify_safe_encoded

else
    log_warn "Running in UNSAFE MODE"
    if [[ -n "$WEBHOOK_URL" ]]; then
        log_info "Sending to webhook: $WEBHOOK_URL"
        curl -s -X POST "$WEBHOOK_URL" -d "message=$MESSAGE"
    fi

    # vuln-code-snippet start dfo_notify_json_injection
    if [[ -n "$SLACK_WEBHOOK" ]]; then
        log_info "Sending to Slack"
        curl -s -X POST "$SLACK_WEBHOOK" \
            -H "Content-Type: application/json" \
            -d "{\"text\": \"$MESSAGE\"}"  # vuln-code-snippet vuln-line dfo_notify_json_injection
    fi
    # vuln-code-snippet end dfo_notify_json_injection

    # Custom webhook
    if [[ -n "$CUSTOM_WEBHOOK" ]]; then
        log_info "Sending to custom webhook: $CUSTOM_WEBHOOK"
        curl -s -X POST "$CUSTOM_WEBHOOK" -d "$MESSAGE"
    fi

    # vuln-code-snippet start dfo_notify_mail_unquoted
    if [[ -n "$EMAIL_TO" ]]; then
        log_info "Sending email to: $EMAIL_TO"
        echo "$MESSAGE" | mail -s "DeployBot Notification" $EMAIL_TO  # vuln-code-snippet vuln-line dfo_notify_mail_unquoted
    fi
    # vuln-code-snippet end dfo_notify_mail_unquoted

    # vuln-code-snippet start dfo_notify_eval_callback
    if [[ -n "$NOTIFY_CALLBACK" ]]; then
        log_info "Executing notification callback"
        eval "$NOTIFY_CALLBACK"  # vuln-code-snippet vuln-line dfo_notify_eval_callback
    fi
    # vuln-code-snippet end dfo_notify_eval_callback
fi

log_info "Notification completed"
exit 0
