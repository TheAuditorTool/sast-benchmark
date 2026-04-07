#!/bin/bash
# Input validation and sanitization functions

# Validate action against whitelist (SAFE - case statement)
# Sanitizer: case statement whitelist
validate_action() {
    local action="$1"

    case "$action" in
        deploy|notify|status|rollback|health)
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

# Validate path using realpath (SAFE - canonicalization)
# Sanitizer: realpath canonicalization
validate_path() {
    local path="$1"
    local base_dir="$2"

    # Canonicalize the path
    local canonical
    canonical=$(realpath -m "$path" 2>/dev/null)

    # Check if path is under base directory
    if [[ "$canonical" == "$base_dir"* ]]; then
        echo "$canonical"
        return 0
    else
        return 1
    fi
}

# Sanitize string for shell usage (SAFE - printf %q)
# Sanitizer: printf %q shell quoting
sanitize_shell() {
    local input="$1"
    printf '%q' "$input"
}

# Validate string against regex pattern (SAFE - [[ =~ ]])
# Sanitizer: regex validation
validate_regex() {
    local input="$1"
    local pattern="$2"

    if [[ "$input" =~ $pattern ]]; then
        return 0
    else
        return 1
    fi
}

# Validate hostname/URL
validate_hostname() {
    local host="$1"
    local pattern='^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$'

    validate_regex "$host" "$pattern"
}

# Validate branch name
validate_branch() {
    local branch="$1"
    local pattern='^[a-zA-Z0-9._/-]+$'

    # Check pattern
    if ! validate_regex "$branch" "$pattern"; then
        return 1
    fi

    # Disallow .. for path traversal
    if [[ "$branch" == *".."* ]]; then
        return 1
    fi

    return 0
}

# Validate integer
validate_integer() {
    local value="$1"
    local min="${2:-}"
    local max="${3:-}"

    # Check if integer
    if ! [[ "$value" =~ ^-?[0-9]+$ ]]; then
        return 1
    fi

    # Check min bound
    if [[ -n "$min" ]] && [[ "$value" -lt "$min" ]]; then
        return 1
    fi

    # Check max bound
    if [[ -n "$max" ]] && [[ "$value" -gt "$max" ]]; then
        return 1
    fi

    return 0
}

# Validate email address
validate_email() {
    local email="$1"
    local pattern='^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'

    validate_regex "$email" "$pattern"
}

# Validate webhook signature (HMAC)
validate_signature() {
    local payload="$1"
    local signature="$2"
    local secret="$3"

    local expected
    expected=$(echo -n "$payload" | openssl dgst -sha256 -hmac "$secret" | awk '{print $2}')

    if [[ "$signature" == "sha256=$expected" ]]; then
        return 0
    else
        return 1
    fi
}

# Escape HTML special characters
escape_html() {
    local input="$1"
    input="${input//&/&amp;}"
    input="${input//</&lt;}"
    input="${input//>/&gt;}"
    input="${input//\"/&quot;}"
    input="${input//\'/&#39;}"
    echo "$input"
}

# Sanitize for SQL (basic escaping - not recommended for real use)
sanitize_sql() {
    local input="$1"
    # Escape single quotes by doubling them
    input="${input//\'/\'\'}"
    echo "$input"
}

# Validate JSON structure has required fields
validate_json_fields() {
    local json="$1"
    shift
    local required_fields=("$@")

    for field in "${required_fields[@]}"; do
        local value
        value=$(echo "$json" | jq -r ".$field // empty" 2>/dev/null)
        if [[ -z "$value" ]]; then
            return 1
        fi
    done

    return 0
}
