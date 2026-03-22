#!/bin/bash
# HTTP request/response parsing utilities

# Parse the HTTP request line
# Returns: METHOD PATH QUERY_STRING HTTP_VERSION
parse_request_line() {
    local request_line="$1"

    # Extract method, full path, and HTTP version
    read -r METHOD FULL_PATH HTTP_VERSION <<< "$request_line"

    # Split path and query string
    if [[ "$FULL_PATH" == *"?"* ]]; then
        PATH_INFO="${FULL_PATH%%\?*}"
        QUERY_STRING="${FULL_PATH#*\?}"
    else
        PATH_INFO="$FULL_PATH"
        QUERY_STRING=""
    fi

    export METHOD PATH_INFO QUERY_STRING HTTP_VERSION
}

# Parse HTTP headers into an associative array
# Usage: parse_headers
# Headers are read from stdin until empty line
parse_headers() {
    declare -gA HEADERS

    while IFS= read -r line; do
        # Remove carriage return
        line="${line%$'\r'}"

        # Empty line signals end of headers
        [[ -z "$line" ]] && break

        # Parse header: name: value
        if [[ "$line" == *":"* ]]; then
            local name="${line%%:*}"
            local value="${line#*: }"
            # Normalize header name to lowercase
            name="${name,,}"
            HEADERS["$name"]="$value"
        fi
    done
}

# Read request body based on Content-Length
# Usage: body=$(read_body)
read_body() {
    local content_length="${HEADERS[content-length]:-0}"

    if [[ "$content_length" -gt 0 ]]; then
        # Read exactly content_length bytes
        head -c "$content_length"
    fi
}

# vuln-code-snippet start dfw_parse_qs_eval
# Parse URL-encoded query string into variables
parse_query_string_unsafe() {
    local qs="$1"

    eval "$qs" # vuln-code-snippet vuln-line dfw_parse_qs_eval
}
# vuln-code-snippet end dfw_parse_qs_eval

# vuln-code-snippet start dfw_parse_qs_safe
# Parse query string safely into associative array
parse_query_string_safe() {
    local qs="$1"
    declare -gA QUERY_PARAMS

    # Split on & and parse each param
    IFS='&' read -ra params <<< "$qs"
    for param in "${params[@]}"; do
        local key="${param%%=*}"
        local value="${param#*=}"
        # URL decode
        value=$(printf '%b' "${value//%/\\x}")
        QUERY_PARAMS["$key"]="$value" # vuln-code-snippet safe-line dfw_parse_qs_safe
    done
}
# vuln-code-snippet end dfw_parse_qs_safe

# vuln-code-snippet start dfw_send_response_safe
# Send HTTP response
# Usage: send_response STATUS_CODE CONTENT_TYPE BODY
send_response() {
    local status_code="$1"
    local content_type="${2:-text/plain}"
    local body="$3"

    local status_text
    case "$status_code" in
        200) status_text="OK" ;;
        201) status_text="Created" ;;
        400) status_text="Bad Request" ;;
        401) status_text="Unauthorized" ;;
        403) status_text="Forbidden" ;;
        404) status_text="Not Found" ;;
        500) status_text="Internal Server Error" ;;
        *) status_text="Unknown" ;;
    esac

    local content_length=${#body}

    printf "HTTP/1.1 %s %s\r\n" "$status_code" "$status_text" # vuln-code-snippet safe-line dfw_send_response_safe
    printf "Content-Type: %s\r\n" "$content_type"
    printf "Content-Length: %d\r\n" "$content_length"
    printf "Connection: close\r\n"
    printf "\r\n"
    printf "%s" "$body"
}
# vuln-code-snippet end dfw_send_response_safe

# Send JSON response
send_json_response() {
    local status_code="$1"
    local json_body="$2"

    send_response "$status_code" "application/json" "$json_body"
}

# vuln-code-snippet start dfw_send_error_json_inject
# Send error response
send_error() {
    local status_code="$1"
    local message="$2"

    local json="{\"error\": true, \"message\": \"$message\"}" # vuln-code-snippet vuln-line dfw_send_error_json_inject
    send_json_response "$status_code" "$json"
}
# vuln-code-snippet end dfw_send_error_json_inject

# Extract path segments
# Usage: segments=($(get_path_segments "/api/v1/deploy"))
get_path_segments() {
    local path="$1"
    IFS='/' read -ra segments <<< "$path"
    printf '%s\n' "${segments[@]}"
}
