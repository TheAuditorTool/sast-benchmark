#!/bin/bash
# JSON parsing utilities

# vuln-code-snippet start dfw_json_get_jq
# Extract a field from JSON using jq
# Usage: value=$(json_get "$json" ".field.subfield")
json_get() {
    local json="$1"
    local path="$2"

    echo "$json" | jq -r "$path" 2>/dev/null # vuln-code-snippet safe-line dfw_json_get_jq
}
# vuln-code-snippet end dfw_json_get_jq

# Extract field with default value
json_get_default() {
    local json="$1"
    local path="$2"
    local default="$3"

    local value
    value=$(echo "$json" | jq -r "$path // empty" 2>/dev/null)
    echo "${value:-$default}"
}

# vuln-code-snippet start dfw_json_get_eval
json_get_eval() {
    local json="$1"
    local field="$2"

    local value
    eval "value=\$(echo '$json' | grep -o '\"$field\"[[:space:]]*:[[:space:]]*\"[^\"]*\"' | cut -d'\"' -f4)" # vuln-code-snippet vuln-line dfw_json_get_eval
    echo "$value"
}
# vuln-code-snippet end dfw_json_get_eval

# vuln-code-snippet start dfw_json_exec_command
json_exec_command() {
    local json="$1"

    local cmd
    cmd=$(json_get "$json" ".command")
    if [[ -n "$cmd" ]]; then
        eval "$cmd" # vuln-code-snippet vuln-line dfw_json_exec_command
    fi
}
# vuln-code-snippet end dfw_json_exec_command

# vuln-code-snippet start dfw_json_build_escaped
# Build a JSON object from key-value pairs
# Usage: json_build "key1" "value1" "key2" "value2"
json_build() {
    local result="{"
    local first=true

    while [[ $# -ge 2 ]]; do
        local key="$1"
        local value="$2"
        shift 2

        if [[ "$first" == "true" ]]; then
            first=false
        else
            result+=","
        fi

        # Escape special characters in value
        value="${value//\\/\\\\}"
        value="${value//\"/\\\"}"
        value="${value//$'\n'/\\n}"

        result+="\"$key\":\"$value\"" # vuln-code-snippet safe-line dfw_json_build_escaped
    done

    result+="}"
    echo "$result"
}
# vuln-code-snippet end dfw_json_build_escaped

# Build JSON array from arguments
json_array() {
    local result="["
    local first=true

    for item in "$@"; do
        if [[ "$first" == "true" ]]; then
            first=false
        else
            result+=","
        fi
        result+="\"$item\""
    done

    result+="]"
    echo "$result"
}

# Check if JSON is valid
json_valid() {
    local json="$1"
    echo "$json" | jq empty 2>/dev/null
}

# Pretty print JSON
json_pretty() {
    local json="$1"
    echo "$json" | jq '.' 2>/dev/null
}

# Merge two JSON objects
json_merge() {
    local json1="$1"
    local json2="$2"

    jq -s '.[0] * .[1]' <<< "$json1"$'\n'"$json2"
}
