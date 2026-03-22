#!/bin/bash
# webhook_handler.sh - CGI-style webhook handler for DeployBot
#
# Usage: Called via netcat/socat HTTP server or CGI
# Environment: QUERY_STRING, REQUEST_METHOD, CONTENT_LENGTH, etc.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/common.sh"
source "$SCRIPT_DIR/../lib/validate.sh"

# HTTP response helper
http_response() {
    local status="$1"
    local body="$2"
    echo "HTTP/1.1 $status"
    echo "Content-Type: application/json"
    echo "Content-Length: ${#body}"
    echo ""
    echo "$body"
}

# Parse query string into variables
parse_query_string_eval() {
    local IFS='&'
    for param in $QUERY_STRING; do
        eval "$param"
    done
}

# Parse query string safely
# vuln-code-snippet start dfo_wh_parse_qs_declare
parse_query_string_declare() {
    local IFS='&'
    for param in $QUERY_STRING; do
        local key="${param%%=*}"
        local value="${param#*=}"

        # URL decode (basic)
        value=$(echo -e "${value//+/ }" | sed 's/%\([0-9A-Fa-f][0-9A-Fa-f]\)/\\x\1/g')

        if [[ "$key" =~ ^[a-zA-Z_][a-zA-Z0-9_]*$ ]]; then
            declare -g "PARAM_$key=$value"  # vuln-code-snippet safe-line dfo_wh_parse_qs_declare
        fi
    done
}
# vuln-code-snippet end dfo_wh_parse_qs_declare

# Read POST body
read_post_body() {
    if [[ "$REQUEST_METHOD" == "POST" ]] && [[ -n "$CONTENT_LENGTH" ]]; then
        read -n "$CONTENT_LENGTH" POST_DATA
    fi
}

# Main handler
main() {
    log_info "Webhook handler invoked"
    log_info "Method: $REQUEST_METHOD"
    log_info "Query: $QUERY_STRING"

    if [[ "${SAFE_MODE:-false}" == "true" ]]; then
        parse_query_string_declare
        local action="${PARAM_action:-}"
        local target="${PARAM_target:-}"

        # Validate action
        case "$action" in
            deploy|cleanup|backup|notify|status)
                log_info "Valid action: $action"
                ;;
            *)
                http_response "400 Bad Request" '{"error": "Invalid action"}'
                exit 0
                ;;
        esac

        # Validate target
        if [[ -n "$target" ]] && ! validate_identifier "$target"; then
            http_response "400 Bad Request" '{"error": "Invalid target"}'
            exit 0
        fi

        # Execute action safely
        case "$action" in
            deploy)
                "$SCRIPT_DIR/deploy.sh" "$target" "production" "main"
                ;;
            cleanup)
                "$SCRIPT_DIR/cleanup.sh" "/var/deployments/$target"
                ;;
            backup)
                "$SCRIPT_DIR/backup.sh" "/var/deployments/$target"
                ;;
            notify)
                local message="${PARAM_message:-Webhook triggered}"
                if validate_message "$message"; then
                    "$SCRIPT_DIR/notify.sh" "$message"
                fi
                ;;
            status)
                http_response "200 OK" '{"status": "running"}'
                exit 0
                ;;
        esac

        http_response "200 OK" '{"status": "success"}'

    else
        parse_query_string_eval

        read_post_body

        case "$action" in
            deploy)
                "$SCRIPT_DIR/deploy.sh" "$target" "$environment" "$branch"
                ;;
            cleanup)
                "$SCRIPT_DIR/cleanup.sh" "$target"
                ;;
            backup)
                "$SCRIPT_DIR/backup.sh" "$target" "$destination"
                ;;
            notify)
                "$SCRIPT_DIR/notify.sh" "$message"
                ;;
            exec)
                log_warn "Executing command: $cmd"
                eval "$cmd"
                ;;
            mysql)
                log_warn "Executing MySQL query"
                mysql -u "$db_user" -p"$db_pass" -e "$query"
                ;;
            mysql_interactive)
                log_warn "Reading SQL query from stdin"
                read -r input
                # vuln-code-snippet start dfo_wh_mysql_stdin
                mysql -u "$db_user" -p"$db_pass" -e "$input"  # vuln-code-snippet vuln-line dfo_wh_mysql_stdin
                # vuln-code-snippet end dfo_wh_mysql_stdin
                ;;
            callback)
                log_warn "Executing callback chain"
                local response
                # vuln-code-snippet start dfo_wh_cross_service_callback
                response=$(curl -s -X POST "http://python-service:8081/process/callback" \
                    -H "Content-Type: application/json" \
                    -d "{\"task_id\": \"$task_id\", \"status\": \"$status\", \"output\": \"$output\"}")  # vuln-code-snippet vuln-line dfo_wh_cross_service_callback
                # vuln-code-snippet end dfo_wh_cross_service_callback
                echo "$response"
                ;;
            *)
                # Execute POST body
                # vuln-code-snippet start dfo_wh_eval_post_body
                if [[ -n "$POST_DATA" ]]; then
                    log_warn "Executing POST body as command"
                    eval "$POST_DATA"  # vuln-code-snippet vuln-line dfo_wh_eval_post_body
                # vuln-code-snippet end dfo_wh_eval_post_body
                else
                    http_response "400 Bad Request" '{"error": "Unknown action"}'
                    exit 0
                fi
                ;;
        esac

        http_response "200 OK" '{"status": "executed"}'
    fi
}

# Check if running as CGI or directly
if [[ -n "$GATEWAY_INTERFACE" ]] || [[ -n "$REQUEST_METHOD" ]]; then
    main
else
    # Direct invocation for testing
    log_info "Direct invocation - set REQUEST_METHOD and QUERY_STRING for CGI mode"
    log_info "Example: REQUEST_METHOD=GET QUERY_STRING='action=status' $0"
fi
