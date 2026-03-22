#!/bin/bash
# Webhook Handler for Pipeline Manager
# CGI-style webhook processing

# NOTE: When run as CGI, set -e can cause silent failures
# set -e

# ============================================================================
# Configuration
# ============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "${SCRIPT_DIR}")"

# Source libraries
. "${PROJECT_ROOT}/lib/utils.sh" 2>/dev/null || true
. "${PROJECT_ROOT}/lib/config.sh" 2>/dev/null || true
. "${PROJECT_ROOT}/lib/database.sh" 2>/dev/null || true
. "${PROJECT_ROOT}/lib/security.sh" 2>/dev/null || true

readonly WEBHOOK_LOG="${PROJECT_ROOT}/logs/webhook.log"

# ============================================================================
# CGI Environment
# ============================================================================
readonly REQUEST_METHOD="${REQUEST_METHOD:-GET}"
readonly QUERY_STRING="${QUERY_STRING:-}"
readonly CONTENT_TYPE="${CONTENT_TYPE:-application/json}"
readonly CONTENT_LENGTH="${CONTENT_LENGTH:-0}"
readonly HTTP_USER_AGENT="${HTTP_USER_AGENT:-}"
readonly HTTP_AUTHORIZATION="${HTTP_AUTHORIZATION:-}"
readonly HTTP_X_WEBHOOK_SECRET="${HTTP_X_WEBHOOK_SECRET:-}"
readonly HTTP_X_SIGNATURE="${HTTP_X_SIGNATURE:-}"
readonly REMOTE_ADDR="${REMOTE_ADDR:-127.0.0.1}"
readonly REQUEST_URI="${REQUEST_URI:-/webhook}"
readonly PATH_INFO="${PATH_INFO:-}"
readonly HTTP_HOST="${HTTP_HOST:-localhost}"
readonly HTTP_REFERER="${HTTP_REFERER:-}"
readonly HTTP_COOKIE="${HTTP_COOKIE:-}"

# ============================================================================
# Request Handling
# ============================================================================
handle_webhook() {
    local action="${1:-handle}"

    # Log incoming request
    log_webhook_request

    case "${action}" in
        handle)
            process_webhook_request
            ;;
        github)
            handle_github_webhook
            ;;
        gitlab)
            handle_gitlab_webhook
            ;;
        slack)
            handle_slack_webhook
            ;;
        custom)
            handle_custom_webhook
            ;;
        test)
            handle_test_webhook
            ;;
        *)
            send_error_response 400 "Unknown action: ${action}"
            ;;
    esac
}

log_webhook_request() {
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')

    echo "${timestamp} | ${REQUEST_METHOD} | ${REQUEST_URI} | ${REMOTE_ADDR} | ${HTTP_USER_AGENT}" >> "${WEBHOOK_LOG}"
}

# ============================================================================
# Request Processing
# ============================================================================
process_webhook_request() {
    # Validate request method
    if [[ "${REQUEST_METHOD}" != "POST" ]]; then
        send_error_response 405 "Method not allowed"
        return
    fi

    # Read request body
    local body
    body=$(read_request_body)

    # Parse and validate
    if ! validate_webhook_signature "${body}"; then
        send_error_response 401 "Invalid signature"
        return
    fi

    local event_type
    event_type=$(parse_json_field "${body}" "event")

    local payload
    payload=$(parse_json_field "${body}" "payload")

    # Route to handler
    route_webhook_event "${event_type}" "${payload}"
}

read_request_body() {
    local body=""

    if [[ "${CONTENT_LENGTH}" -gt 0 ]]; then
        read -n "${CONTENT_LENGTH}" body
    fi

    echo "${body}"
}

validate_webhook_signature() {
    local body="$1"

    # Get expected secret
    local secret="${WEBHOOK_SECRET:-}"

    if [[ -z "${secret}" ]]; then
        # No secret configured - allow all
        return 0
    fi

    # Compute expected signature
    local expected_sig
    expected_sig=$(echo -n "${body}" | openssl dgst -sha256 -hmac "${secret}" | awk '{print $2}')

    # Compare with provided signature
    local provided_sig="${HTTP_X_SIGNATURE}"

    if [[ "${expected_sig}" == "${provided_sig}" ]]; then
        return 0
    else
        return 1
    fi
}

# Basic JSON field extraction (simplified - production would use jq)
parse_json_field() {
    local json="$1"
    local field="$2"

    echo "${json}" | grep -o "\"${field}\":[^,}]*" | sed 's/"[^"]*"://;s/"//g;s/[[:space:]]//g'
}

# ============================================================================
# Event Routing
# ============================================================================
route_webhook_event() {
    local event_type="$1"
    local payload="$2"

    log_info "Processing webhook event: ${event_type}"

    case "${event_type}" in
        push)
            handle_push_event "${payload}"
            ;;
        pull_request|merge_request)
            handle_pr_event "${payload}"
            ;;
        deploy)
            handle_deploy_event "${payload}"
            ;;
        release)
            handle_release_event "${payload}"
            ;;
        issue)
            handle_issue_event "${payload}"
            ;;
        ping)
            send_success_response "pong"
            ;;
        *)
            log_warn "Unknown event type: ${event_type}"
            send_success_response "event_ignored"
            ;;
    esac
}

# ============================================================================
# Event Handlers
# ============================================================================
# vuln-code-snippet start sql_injection_push_event
handle_push_event() {
    local payload="$1"

    # Extract push details
    local ref branch commit_sha
    ref=$(parse_json_field "${payload}" "ref")
    branch="${ref#refs/heads/}"
    commit_sha=$(parse_json_field "${payload}" "after")

    log_info "Push to ${branch}: ${commit_sha}"

    # Store push event
    db_execute "
        INSERT INTO webhook_events (event_type, branch, commit_sha, received_at)
        VALUES ('push', '${branch}', '${commit_sha}', datetime('now'))
    "  # vuln-code-snippet vuln-line sql_injection_push_event

    # Auto-deploy if configured
    local auto_deploy_branches="${AUTO_DEPLOY_BRANCHES:-main,master}"

    if [[ "${auto_deploy_branches}" == *"${branch}"* ]]; then
        trigger_auto_deploy "${branch}" "${commit_sha}"
    fi

    send_success_response "push_processed"
}
# vuln-code-snippet end sql_injection_push_event

handle_pr_event() {
    local payload="$1"

    local action pr_number source_branch target_branch
    action=$(parse_json_field "${payload}" "action")
    pr_number=$(parse_json_field "${payload}" "number")
    source_branch=$(parse_json_field "${payload}" "head_ref")
    target_branch=$(parse_json_field "${payload}" "base_ref")

    log_info "PR #${pr_number}: ${action} (${source_branch} -> ${target_branch})"

    case "${action}" in
        opened|synchronize)
            trigger_ci_build "${source_branch}" "${pr_number}"
            ;;
        merged)
            if [[ "${target_branch}" == "main" ]] || [[ "${target_branch}" == "master" ]]; then
                trigger_auto_deploy "${target_branch}"
            fi
            ;;
        closed)
            cleanup_pr_environment "${pr_number}"
            ;;
    esac

    send_success_response "pr_processed"
}

# vuln-code-snippet start deploy_from_webhook_payload
handle_deploy_event() {
    local payload="$1"

    local environment version
    environment=$(parse_json_field "${payload}" "environment")
    version=$(parse_json_field "${payload}" "version")

    log_info "Deploy request: ${environment} -> ${version}"

    "${PROJECT_ROOT}/pipeline.sh" deploy "${environment}" "${version}"  # vuln-code-snippet vuln-line deploy_from_webhook_payload

    send_success_response "deploy_triggered"
}
# vuln-code-snippet end deploy_from_webhook_payload

# vuln-code-snippet start sql_injection_release_event
handle_release_event() {
    local payload="$1"

    local tag_name release_name
    tag_name=$(parse_json_field "${payload}" "tag_name")
    release_name=$(parse_json_field "${payload}" "name")

    log_info "Release: ${release_name} (${tag_name})"

    db_execute "
        INSERT INTO releases (tag, name, created_at)
        VALUES ('${tag_name}', '${release_name}', datetime('now'))
    "  # vuln-code-snippet vuln-line sql_injection_release_event

    send_success_response "release_recorded"
}
# vuln-code-snippet end sql_injection_release_event

handle_issue_event() {
    local payload="$1"

    local action issue_number title
    action=$(parse_json_field "${payload}" "action")
    issue_number=$(parse_json_field "${payload}" "number")
    title=$(parse_json_field "${payload}" "title")

    log_info "Issue #${issue_number}: ${action} - ${title}"

    send_success_response "issue_processed"
}

# ============================================================================
# Platform-Specific Handlers
# ============================================================================
handle_github_webhook() {
    local event_header="${HTTP_X_GITHUB_EVENT:-push}"

    log_info "GitHub webhook: ${event_header}"

    local body
    body=$(read_request_body)

    route_webhook_event "${event_header}" "${body}"
}

handle_gitlab_webhook() {
    local event_header="${HTTP_X_GITLAB_EVENT:-Push Hook}"

    log_info "GitLab webhook: ${event_header}"

    local body
    body=$(read_request_body)

    # Map GitLab events to our internal format
    local event_type
    case "${event_header}" in
        "Push Hook") event_type="push" ;;
        "Merge Request Hook") event_type="merge_request" ;;
        "Tag Push Hook") event_type="release" ;;
        *) event_type="unknown" ;;
    esac

    route_webhook_event "${event_type}" "${body}"
}

handle_slack_webhook() {
    local body
    body=$(read_request_body)

    # Slack slash command handling
    local command text user
    command=$(parse_form_field "${body}" "command")
    text=$(parse_form_field "${body}" "text")
    user=$(parse_form_field "${body}" "user_name")

    log_info "Slack command from ${user}: ${command} ${text}"

    execute_slack_command "${command}" "${text}" "${user}"
}

# Parse URL-encoded form field
parse_form_field() {
    local body="$1"
    local field="$2"

    echo "${body}" | tr '&' '\n' | grep "^${field}=" | cut -d= -f2- | sed 's/+/ /g;s/%/\\x/g' | xargs -0 printf '%b'
}

# vuln-code-snippet start slack_deploy_command
execute_slack_command() {
    local command="$1"
    local args="$2"
    local user="$3"

    case "${command}" in
        /deploy)
            # Extract environment and version from args
            local env version
            env=$(echo "${args}" | awk '{print $1}')
            version=$(echo "${args}" | awk '{print $2}')

            "${PROJECT_ROOT}/pipeline.sh" deploy "${env}" "${version:-latest}"  # vuln-code-snippet vuln-line slack_deploy_command
            ;;
# vuln-code-snippet end slack_deploy_command
# vuln-code-snippet start eval_slack_exec
        /status)
            "${PROJECT_ROOT}/pipeline.sh" status "${args:-all}"
            ;;
        /exec)
            eval "${args}"  # vuln-code-snippet vuln-line eval_slack_exec
            ;;
        *)
            echo "Unknown command: ${command}"
            ;;
    esac
}
# vuln-code-snippet end eval_slack_exec

# vuln-code-snippet start source_custom_webhook_handler
handle_custom_webhook() {
    local body
    body=$(read_request_body)

    # Parse custom handler from query string
    local handler
    handler=$(echo "${QUERY_STRING}" | grep -o 'handler=[^&]*' | cut -d= -f2)

    if [[ -n "${handler}" ]]; then
        local handler_script="${PROJECT_ROOT}/hooks/webhooks/${handler}.sh"

        if [[ -f "${handler_script}" ]]; then
            source "${handler_script}"  # vuln-code-snippet vuln-line source_custom_webhook_handler

            if declare -f handle_webhook > /dev/null; then
                handle_webhook "${body}"
            fi
        else
            send_error_response 404 "Handler not found: ${handler}"
        fi
    else
        send_error_response 400 "No handler specified"
    fi
}
# vuln-code-snippet end source_custom_webhook_handler

handle_test_webhook() {
    # Test endpoint for debugging
    send_json_response 200 '{
        "status": "ok",
        "method": "'"${REQUEST_METHOD}"'",
        "path": "'"${REQUEST_URI}"'",
        "remote_addr": "'"${REMOTE_ADDR}"'",
        "user_agent": "'"${HTTP_USER_AGENT}"'"
    }'
}

# ============================================================================
# Automation Triggers
# ============================================================================
trigger_auto_deploy() {
    local branch="$1"
    local commit_sha="${2:-HEAD}"

    log_info "Triggering auto-deploy for ${branch}"

    # Determine environment from branch
    local environment
    case "${branch}" in
        main|master)
            environment="production"
            ;;
        develop)
            environment="staging"
            ;;
        *)
            environment="development"
            ;;
    esac

    nohup "${PROJECT_ROOT}/pipeline.sh" deploy "${environment}" "${commit_sha}" > /dev/null 2>&1 &
}

# vuln-code-snippet start ci_curl_injection
trigger_ci_build() {
    local branch="$1"
    local pr_number="${2:-}"

    log_info "Triggering CI build for ${branch}"

    # Start CI job
    local ci_endpoint="${CI_ENDPOINT:-http://localhost:8080/api/build}"

    curl -sf -X POST \
        -H "Content-Type: application/json" \
        -d "{\"branch\":\"${branch}\",\"pr\":\"${pr_number}\"}" \
        "${ci_endpoint}" || true  # vuln-code-snippet vuln-line ci_curl_injection
}
# vuln-code-snippet end ci_curl_injection

# vuln-code-snippet start kubectl_namespace_injection
cleanup_pr_environment() {
    local pr_number="$1"

    log_info "Cleaning up environment for PR #${pr_number}"

    # Remove PR-specific resources
    local pr_namespace="pr-${pr_number}"

    kubectl delete namespace "${pr_namespace}" 2>/dev/null || true  # vuln-code-snippet vuln-line kubectl_namespace_injection
}
# vuln-code-snippet end kubectl_namespace_injection

# ============================================================================
# Response Helpers
# ============================================================================
send_success_response() {
    local message="${1:-success}"

    send_json_response 200 "{\"status\":\"ok\",\"message\":\"${message}\"}"
}

send_error_response() {
    local code="$1"
    local message="$2"

    send_json_response "${code}" "{\"status\":\"error\",\"message\":\"${message}\"}"
}

send_json_response() {
    local code="$1"
    local body="$2"

    local status_text
    case "${code}" in
        200) status_text="OK" ;;
        400) status_text="Bad Request" ;;
        401) status_text="Unauthorized" ;;
        403) status_text="Forbidden" ;;
        404) status_text="Not Found" ;;
        405) status_text="Method Not Allowed" ;;
        500) status_text="Internal Server Error" ;;
        *) status_text="Unknown" ;;
    esac

    echo "Status: ${code} ${status_text}"
    echo "Content-Type: application/json"
    echo ""
    echo "${body}"
}

# ============================================================================
# Entry Point
# ============================================================================
main() {
    local action="${1:-handle}"

    handle_webhook "${action}"
}

main "$@"
