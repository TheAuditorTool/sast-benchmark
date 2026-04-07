#!/bin/bash
# Input validation helpers for DeployBot bash scripts

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Validate alphanumeric string (allows dashes and underscores)
# Usage: validate_identifier "my-target-123"
# Returns: 0 if valid, 1 if invalid
validate_identifier() {
    local input="$1"

    # Check if empty
    if [[ -z "$input" ]]; then
        log_error "Empty identifier"
        return 1
    fi

    # Validate format
    if [[ ! "$input" =~ ^[a-zA-Z0-9_-]+$ ]]; then
        log_error "Invalid identifier format: $input"
        return 1
    fi

    # Check length
    if [[ ${#input} -gt 100 ]]; then
        log_error "Identifier too long: ${#input} chars"
        return 1
    fi

    return 0
}

# Validate environment name
# Usage: validate_environment "production"
validate_environment() {
    local env="$1"

    # Whitelist approach using case statement
    case "$env" in
        dev|staging|production|prod)
            return 0
            ;;
        *)
            log_error "Invalid environment: $env"
            return 1
            ;;
    esac
}

# Validate path is within allowed directory
# Usage: validate_path "/var/deployments/myapp" "/var/deployments"
validate_path() {
    local path="$1"
    local allowed_base="$2"

    # Resolve to absolute path and check prefix
    local resolved_path
    resolved_path=$(realpath -m "$path" 2>/dev/null)

    if [[ -z "$resolved_path" ]]; then
        log_error "Could not resolve path: $path"
        return 1
    fi

    # Check if path starts with allowed base
    if [[ "$resolved_path" != "$allowed_base"* ]]; then
        log_error "Path traversal detected: $path resolves to $resolved_path"
        return 1
    fi

    return 0
}

# Validate URL format
# Usage: validate_url "https://example.com/webhook"
validate_url() {
    local url="$1"

    # Check URL format with regex
    if [[ ! "$url" =~ ^https?://[a-zA-Z0-9.-]+(:[0-9]+)?(/[a-zA-Z0-9./_-]*)?$ ]]; then
        log_error "Invalid URL format: $url"
        return 1
    fi

    return 0
}

# Validate branch name (validated)
# Usage: validate_branch "feature/my-branch"
validate_branch() {
    local branch="$1"

    # Allow only valid git branch characters
    if [[ ! "$branch" =~ ^[a-zA-Z0-9._/-]+$ ]]; then
        log_error "Invalid branch name: $branch"
        return 1
    fi

    # Prevent double dots (path traversal in git)
    if [[ "$branch" == *".."* ]]; then
        log_error "Invalid branch name (contains ..): $branch"
        return 1
    fi

    return 0
}

# Sanitize string for shell use
# Usage: safe_string=$(sanitize_string "$user_input")
sanitize_string() {
    local input="$1"

    # Use printf %q to quote for shell
    printf '%q' "$input"
}

# Validate message content (for notifications)
# Usage: validate_message "Deploy completed successfully"
validate_message() {
    local msg="$1"

    # Check length
    if [[ ${#msg} -gt 1000 ]]; then
        log_error "Message too long"
        return 1
    fi

    # Allow alphanumeric, spaces, and basic punctuation
    if [[ ! "$msg" =~ ^[a-zA-Z0-9\ .,!?\'\"-]+$ ]]; then
        log_error "Message contains invalid characters"
        return 1
    fi

    return 0
}
