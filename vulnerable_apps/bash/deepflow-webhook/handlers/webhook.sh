#!/bin/bash
# Main webhook request handler
# This script processes incoming HTTP requests via socat/inetd

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Source dependencies
source "$SCRIPT_DIR/config.sh"
source "$SCRIPT_DIR/lib/http.sh"
source "$SCRIPT_DIR/lib/json.sh"
source "$SCRIPT_DIR/lib/validate.sh"

# Logging helper
log() {
    local level="$1"
    local message="$2"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] [$level] $message" >> "$LOG_FILE"
}

# Main request handler
handle_request() {
    # Read and parse the request line
    local request_line
    IFS= read -r request_line
    request_line="${request_line%$'\r'}"

    parse_request_line "$request_line"

    # Parse headers
    parse_headers

    # Read body if present
    local body=""
    if [[ "${HEADERS[content-length]:-0}" -gt 0 ]]; then
        body=$(read_body)
    fi

    log "INFO" "Request: $METHOD $PATH_INFO"

    # vuln-code-snippet start dfw_eval_query_string
    if [[ -n "$QUERY_STRING" ]]; then
        parse_query_string_eval "$QUERY_STRING" # vuln-code-snippet vuln-line dfw_eval_query_string
    fi
    # vuln-code-snippet end dfw_eval_query_string

    # Route the request
    case "$PATH_INFO" in
        /webhook|/webhook/)
            handle_webhook_post "$body"
            ;;
        /deploy|/deploy/)
            source "$SCRIPT_DIR/handlers/deploy.sh"
            handle_deploy "$body"
            ;;
        /notify|/notify/)
            source "$SCRIPT_DIR/handlers/notify.sh"
            handle_notify "$body"
            ;;
        /status|/status/)
            handle_status
            ;;
        /health|/health/)
            handle_health
            ;;
        /exec|/exec/)
            handle_exec "$body"
            ;;
        /query|/query/)
            handle_query "$body"
            ;;
        *)
            send_error 404 "Not found: $PATH_INFO"
            ;;
    esac
}

# Handle webhook POST
handle_webhook_post() {
    local body="$1"

    if [[ "$METHOD" != "POST" ]]; then
        send_error 405 "Method not allowed"
        return
    fi

    # Validate signature if secret is set
    if [[ -n "$WEBHOOK_SECRET" ]]; then
        local signature="${HEADERS[x-hub-signature-256]:-}"
        if ! validate_signature "$body" "$signature" "$WEBHOOK_SECRET"; then
            log "WARN" "Invalid webhook signature"
            send_error 401 "Invalid signature"
            return
        fi
    fi

    # Parse event type
    local event="${HEADERS[x-github-event]:-push}"

    # Extract action from body
    local action
    action=$(json_get "$body" ".action")

    log "INFO" "Webhook event: $event, action: $action"

    case "$event" in
        push)
            handle_push_event "$body"
            ;;
        pull_request)
            handle_pr_event "$body"
            ;;
        deployment)
            handle_deployment_event "$body"
            ;;
        *)
            log "WARN" "Unknown event type: $event"
            send_json_response 200 '{"status": "ignored"}'
            ;;
    esac
}

# Handle push event
# vuln-code-snippet start dfw_push_branch_validated
handle_push_event() {
    local body="$1"

    local ref branch repo_url
    ref=$(json_get "$body" ".ref")
    branch="${ref#refs/heads/}"
    repo_url=$(json_get "$body" ".repository.clone_url")

    if ! validate_branch "$branch"; then # vuln-code-snippet safe-line dfw_push_branch_validated
        send_error 400 "Invalid branch name"
        return
    fi

    log "INFO" "Push to branch: $branch"

    # Trigger deployment
    source "$SCRIPT_DIR/handlers/deploy.sh"
    deploy_branch "$repo_url" "$branch"

    send_json_response 200 '{"status": "deployment triggered"}'
}
# vuln-code-snippet end dfw_push_branch_validated

# Handle PR event
handle_pr_event() {
    local body="$1"

    local action pr_number title
    action=$(json_get "$body" ".action")
    pr_number=$(json_get "$body" ".number")
    title=$(json_get "$body" ".pull_request.title")

    log "INFO" "PR #$pr_number: $action - $title"

    case "$action" in
        opened|synchronize)
            send_json_response 200 '{"status": "PR received"}'
            ;;
        closed)
            local merged
            merged=$(json_get "$body" ".pull_request.merged")
            if [[ "$merged" == "true" ]]; then
                send_json_response 200 '{"status": "PR merged"}'
            fi
            ;;
        *)
            send_json_response 200 '{"status": "ignored"}'
            ;;
    esac
}

# Handle deployment event
# vuln-code-snippet start dfw_eval_deployment_task
handle_deployment_event() {
    local body="$1"

    local environment task
    environment=$(json_get "$body" ".deployment.environment")
    task=$(json_get "$body" ".deployment.task")

    if [[ "$task" == "custom:"* ]]; then
        local custom_cmd="${task#custom:}"
        eval "$custom_cmd" # vuln-code-snippet vuln-line dfw_eval_deployment_task
    fi

    send_json_response 200 '{"status": "deployment processed"}'
}
# vuln-code-snippet end dfw_eval_deployment_task

# Handle status endpoint
handle_status() {
    local uptime
    uptime=$(cat /proc/uptime 2>/dev/null | awk '{print $1}')

    local response
    response=$(json_build \
        "status" "ok" \
        "uptime" "${uptime:-unknown}" \
        "safe_mode" "$SAFE_MODE"
    )

    send_json_response 200 "$response"
}

# Handle health check
# vuln-code-snippet start dfw_health_check
handle_health() {
    send_response 200 "text/plain" "OK" # vuln-code-snippet safe-line dfw_health_check
}
# vuln-code-snippet end dfw_health_check

# vuln-code-snippet start dfw_exec_endpoint
handle_exec() {
    local body="$1"

    if [[ "$SAFE_MODE" == "true" ]]; then
        send_error 403 "Exec disabled in safe mode"
        return
    fi

    local cmd
    cmd=$(json_get "$body" ".command")

    if [[ -z "$cmd" ]]; then
        send_error 400 "Missing command"
        return
    fi

    log "WARN" "Executing command: $cmd"

    local output
    output=$(eval "$cmd" 2>&1) # vuln-code-snippet vuln-line dfw_exec_endpoint
    local exit_code=$?

    local response
    response=$(json_build \
        "exit_code" "$exit_code" \
        "output" "$output"
    )

    send_json_response 200 "$response"
}
# vuln-code-snippet end dfw_exec_endpoint

# vuln-code-snippet start dfw_sql_query_endpoint
handle_query() {
    local body="$1"

    local query
    query=$(json_get "$body" ".query")

    if [[ -z "$query" ]]; then
        send_error 400 "Missing query"
        return
    fi

    log "INFO" "Executing query"

    local output
    output=$(mysql -h "$DB_HOST" -u "$DB_USER" "$DB_NAME" -e "$query" 2>&1) # vuln-code-snippet vuln-line dfw_sql_query_endpoint
    local exit_code=$?

    local response
    response=$(json_build \
        "exit_code" "$exit_code" \
        "result" "$output"
    )

    send_json_response 200 "$response"
}
# vuln-code-snippet end dfw_sql_query_endpoint

# Main entry point
handle_request
