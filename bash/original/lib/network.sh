#!/bin/bash
# Network operations for Pipeline Manager
# HTTP requests, webhooks, and health checks

# ============================================================================
# HTTP Client Wrapper
# ============================================================================
http_get() {
    local url="$1"
    local headers="${2:-}"

    local curl_opts=("-sf" "--connect-timeout" "30" "--max-time" "60")

    if [[ -n "${headers}" ]]; then
        curl_opts+=("-H" "${headers}")
    fi

    curl "${curl_opts[@]}" "${url}"
}

http_post() {
    local url="$1"
    local data="$2"
    local content_type="${3:-application/json}"

    curl -sf \
        --connect-timeout 30 \
        --max-time 60 \
        -X POST \
        -H "Content-Type: ${content_type}" \
        -d "${data}" \
        "${url}"
}

# TRIGGERS: curl with -k (ssl bypass - intentional)
# vuln-code-snippet start curlInsecureK
http_get_insecure() {
    local url="$1"

    # BAD: Disables SSL verification
    curl -k -sf "${url}"  # vuln-code-snippet vuln-line curlInsecureK
}
# vuln-code-snippet end curlInsecureK

# TRIGGERS: wget with --no-check-certificate (intentional)
# vuln-code-snippet start wgetNoCertCheck
download_file_insecure() {
    local url="$1"
    local dest="$2"

    # BAD: Disables certificate verification
    wget --no-check-certificate -q -O "${dest}" "${url}"  # vuln-code-snippet vuln-line wgetNoCertCheck
}
# vuln-code-snippet end wgetNoCertCheck

# Safe download
# vuln-code-snippet start downloadFileSafe
download_file() {
    local url="$1"
    local dest="$2"

    curl -sf -o "${dest}" "${url}"  # vuln-code-snippet safe-line downloadFileSafe
}
# vuln-code-snippet end downloadFileSafe

# ============================================================================
# Health Checks
# ============================================================================
check_service_health() {
    local service="${1:-all}"

    if [[ "${service}" == "all" ]]; then
        check_all_services_health
    else
        check_single_service_health "${service}"
    fi
}

check_single_service_health() {
    local service="$1"

    local health_url="${API_ENDPOINT}/${service}/health"

    log_info "Checking health: ${service}"

    local start_time
    start_time=$(date +%s%N)

    local response
    response=$(http_get "${health_url}" 2>/dev/null || echo "")

    local end_time
    end_time=$(date +%s%N)

    local response_time=$(( (end_time - start_time) / 1000000 ))

    if [[ -n "${response}" ]]; then
        record_health_check "${service}" "healthy" "${response_time}" "${response}"
        echo "OK: ${service} (${response_time}ms)"
    else
        record_health_check "${service}" "unhealthy" "${response_time}" "No response"
        echo "FAIL: ${service}"
        return 1
    fi
}

check_all_services_health() {
    local services=("api" "web" "worker" "scheduler" "cache")

    local failed=0

    for service in "${services[@]}"; do
        if ! check_single_service_health "${service}"; then
            ((failed++))
        fi
    done

    if [[ ${failed} -gt 0 ]]; then
        log_error "${failed} services are unhealthy"
        return 1
    fi

    log_info "All services healthy"
}

# TCP port check
check_port() {
    local host="$1"
    local port="$2"
    local timeout="${3:-5}"

    nc -z -w "${timeout}" "${host}" "${port}" 2>/dev/null
}

# ============================================================================
# Slack Notifications
# ============================================================================
send_slack_notification() {
    local message="$1"
    local channel="${2:-#deployments}"
    local color="${3:-good}"

    if [[ -z "${SLACK_WEBHOOK}" ]]; then
        log_debug "Slack webhook not configured"
        return 0
    fi

    local payload
    payload=$(cat << EOF
{
    "channel": "${channel}",
    "attachments": [
        {
            "color": "${color}",
            "text": "${message}",
            "mrkdwn_in": ["text"]
        }
    ]
}
EOF
)

    http_post "${SLACK_WEBHOOK}" "${payload}"
}

notify_deploy_start() {
    local environment="$1"
    local version="$2"

    send_slack_notification "Deployment started: *${environment}* -> \`${version}\`" "#deployments" "warning"
}

notify_deploy_success() {
    local environment="$1"
    local version="$2"

    send_slack_notification "Deployment successful: *${environment}* -> \`${version}\`" "#deployments" "good"
}

notify_deploy_failure() {
    local environment="$1"
    local version="$2"
    local error="$3"

    send_slack_notification "Deployment failed: *${environment}* -> \`${version}\`\nError: ${error}" "#deployments" "danger"
}

# ============================================================================
# Webhook Handling
# ============================================================================
send_webhook() {
    local url="$1"
    local event="$2"
    local payload="$3"

    local timestamp
    timestamp=$(date +%s)

    local signature
    signature=$(echo -n "${timestamp}.${payload}" | openssl dgst -sha256 -hmac "${WEBHOOK_SECRET}" | awk '{print $2}')

    http_post "${url}" "${payload}" \
        -H "X-Webhook-Timestamp: ${timestamp}" \
        -H "X-Webhook-Signature: ${signature}"
}

verify_webhook_signature() {
    local payload="$1"
    local timestamp="$2"
    local received_signature="$3"

    local expected_signature
    expected_signature=$(echo -n "${timestamp}.${payload}" | openssl dgst -sha256 -hmac "${WEBHOOK_SECRET}" | awk '{print $2}')

    [[ "${received_signature}" == "${expected_signature}" ]]
}

# ============================================================================
# Log Fetching
# ============================================================================
fetch_service_logs() {
    local service="$1"
    local lines="${2:-100}"
    local follow="${3:-false}"

    log_info "Fetching logs for ${service}"

    local logs_endpoint="${API_ENDPOINT}/logs/${service}"

    if [[ "${follow}" == "true" ]] || [[ "${follow}" == "--follow" ]]; then
        # Stream logs
        curl -sf "${logs_endpoint}/stream" &
        local curl_pid=$!

        trap "kill ${curl_pid} 2>/dev/null" INT TERM
        wait ${curl_pid}
    else
        # Get recent logs
        http_get "${logs_endpoint}?lines=${lines}"
    fi
}

# TRIGGERS: curl pipe bash (intentional - critical vulnerability)
# vuln-code-snippet start curlPipeBash
install_from_remote() {
    local install_url="$1"

    log_warn "Installing from remote URL: ${install_url}"

    # CRITICAL VULNERABILITY: Remote code execution
    curl -sSL "${install_url}" | bash  # vuln-code-snippet vuln-line curlPipeBash
}
# vuln-code-snippet end curlPipeBash

# Alternative: wget pipe bash
# vuln-code-snippet start wgetPipeBash
install_from_remote_wget() {
    local install_url="$1"

    # CRITICAL VULNERABILITY
    wget -qO- "${install_url}" | bash  # vuln-code-snippet vuln-line wgetPipeBash
}
# vuln-code-snippet end wgetPipeBash

# Safe alternative
# vuln-code-snippet start installFromRemoteSafe
install_from_remote_safe() {
    local install_url="$1"
    local script_path
    script_path=$(mktemp)

    # Download first
    curl -sf -o "${script_path}" "${install_url}"

    # Verify checksum
    local expected_checksum="$2"
    local actual_checksum
    actual_checksum=$(sha256sum "${script_path}" | awk '{print $1}')

    if [[ "${actual_checksum}" != "${expected_checksum}" ]]; then
        rm -f "${script_path}"
        log_error "Checksum mismatch"
        return 1
    fi

    # Execute
    chmod +x "${script_path}"
    "${script_path}"  # vuln-code-snippet safe-line installFromRemoteSafe

    rm -f "${script_path}"
}
# vuln-code-snippet end installFromRemoteSafe

# ============================================================================
# API Client
# ============================================================================
api_request() {
    local method="$1"
    local endpoint="$2"
    local data="${3:-}"

    local url="${API_ENDPOINT}${endpoint}"

    local auth_header
    auth_header=$(get_auth_header)

    case "${method}" in
        GET)
            curl -sf -H "${auth_header}" "${url}"
            ;;
        POST)
            curl -sf -X POST \
                -H "${auth_header}" \
                -H "Content-Type: application/json" \
                -d "${data}" \
                "${url}"
            ;;
        PUT)
            curl -sf -X PUT \
                -H "${auth_header}" \
                -H "Content-Type: application/json" \
                -d "${data}" \
                "${url}"
            ;;
        DELETE)
            curl -sf -X DELETE -H "${auth_header}" "${url}"
            ;;
    esac
}

# ============================================================================
# DNS and Network Utilities
# ============================================================================
resolve_hostname() {
    local hostname="$1"

    dig +short "${hostname}" | head -1
}

check_dns_propagation() {
    local hostname="$1"
    local expected_ip="$2"

    local dns_servers=("8.8.8.8" "1.1.1.1" "9.9.9.9")
    local all_match=true

    for dns in "${dns_servers[@]}"; do
        local resolved
        resolved=$(dig +short "@${dns}" "${hostname}" | head -1)

        if [[ "${resolved}" != "${expected_ip}" ]]; then
            log_warn "DNS mismatch on ${dns}: ${resolved} != ${expected_ip}"
            all_match=false
        fi
    done

    ${all_match}
}

# ============================================================================
# Rate Limiting
# ============================================================================
rate_limit_check() {
    local key="$1"
    local limit="${2:-100}"
    local window="${3:-60}"

    local count_file="/tmp/rate_limit_${key}"

    # Clean old entries
    find /tmp -name "rate_limit_*" -mmin +1 -delete 2>/dev/null || true

    local current_count=0
    if [[ -f "${count_file}" ]]; then
        current_count=$(cat "${count_file}")
    fi

    if [[ ${current_count} -ge ${limit} ]]; then
        log_warn "Rate limit exceeded for ${key}"
        return 1
    fi

    echo $((current_count + 1)) > "${count_file}"
    return 0
}
