#!/bin/bash
# Deployment handler for deepflow-webhook

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Source dependencies if not already loaded
[[ -z "$DEPLOY_DIR" ]] && source "$SCRIPT_DIR/config.sh"
[[ "$(type -t json_get)" != "function" ]] && source "$SCRIPT_DIR/lib/json.sh"
[[ "$(type -t validate_path)" != "function" ]] && source "$SCRIPT_DIR/lib/validate.sh"
[[ "$(type -t send_json_response)" != "function" ]] && source "$SCRIPT_DIR/lib/http.sh"

# Deploy a specific branch
# vuln-code-snippet start dfw_git_clone_ssrf
deploy_branch() {
    local repo_url="$1"
    local branch="$2"

    local target_dir="$DEPLOY_DIR/$branch"

    if ! validate_branch "$branch"; then
        log "ERROR" "Invalid branch name: $branch"
        return 1
    fi

    # Create deployment directory
    mkdir -p "$target_dir"

    git clone --branch "$branch" --single-branch "$repo_url" "$target_dir" 2>&1 # vuln-code-snippet vuln-line dfw_git_clone_ssrf

    log "INFO" "Deployed $branch to $target_dir"
}
# vuln-code-snippet end dfw_git_clone_ssrf

# Handle deploy API endpoint
handle_deploy() {
    local body="$1"

    if [[ "$METHOD" != "POST" ]]; then
        send_error 405 "Method not allowed"
        return
    fi

    local target action script
    target=$(json_get "$body" ".target")
    action=$(json_get "$body" ".action")
    script=$(json_get "$body" ".script")

    if ! validate_action "$action"; then
        send_error 400 "Invalid action"
        return
    fi

    case "$action" in
        deploy)
            do_deploy "$target" "$script"
            ;;
        rollback)
            do_rollback "$target"
            ;;
        status)
            do_deploy_status "$target"
            ;;
        *)
            send_error 400 "Unknown action"
            ;;
    esac
}

# Perform deployment
do_deploy() {
    local target="$1"
    local script="$2"

    local deploy_path="$DEPLOY_DIR/$target"

    # Clean existing deployment
    # vuln-code-snippet start dfw_deploy_path_traversal
    rm -rf "$deploy_path" # vuln-code-snippet vuln-line dfw_deploy_path_traversal
    mkdir -p "$deploy_path"
    # vuln-code-snippet end dfw_deploy_path_traversal

    log "INFO" "Deploying to: $deploy_path"

    # Run pre-deploy script if specified
    # vuln-code-snippet start dfw_deploy_bash_c
    if [[ -n "$script" ]]; then
        log "WARN" "Running deploy script"
        bash -c "$script" # vuln-code-snippet vuln-line dfw_deploy_bash_c
    fi
    # vuln-code-snippet end dfw_deploy_bash_c

    send_json_response 200 '{"status": "deployed"}'
}

# Perform deployment with validation (SAFE version)
# vuln-code-snippet start dfw_deploy_safe_validated
do_deploy_safe() {
    local target="$1"
    local script="$2"

    local deploy_path
    deploy_path=$(validate_path "$DEPLOY_DIR/$target" "$DEPLOY_DIR") # vuln-code-snippet safe-line dfw_deploy_safe_validated

    if [[ $? -ne 0 ]]; then
        send_error 400 "Invalid target path"
        return
    fi

    # Clean existing deployment
    rm -rf "$deploy_path"
    mkdir -p "$deploy_path"

    if [[ -n "$script" ]]; then
        case "$script" in
            npm_install)
                cd "$deploy_path" && npm install
                ;;
            build)
                cd "$deploy_path" && npm run build
                ;;
            *)
                log "WARN" "Unknown script: $script"
                ;;
        esac
    fi

    send_json_response 200 '{"status": "deployed"}'
}
# vuln-code-snippet end dfw_deploy_safe_validated

# Rollback deployment
# vuln-code-snippet start dfw_rollback_path_traversal
do_rollback() {
    local target="$1"

    local deploy_path="$DEPLOY_DIR/$target"
    local backup_path="$DEPLOY_DIR/.backups/$target"

    if [[ -d "$backup_path" ]]; then
        rm -rf "$deploy_path"
        cp -r "$backup_path" "$deploy_path" # vuln-code-snippet vuln-line dfw_rollback_path_traversal
        send_json_response 200 '{"status": "rolled back"}'
    else
        send_error 404 "No backup found"
    fi
}
# vuln-code-snippet end dfw_rollback_path_traversal

# Get deployment status
do_deploy_status() {
    local target="$1"

    local deploy_path="$DEPLOY_DIR/$target"

    if [[ -d "$deploy_path" ]]; then
        local files
        files=$(ls -la "$deploy_path" 2>&1)

        local response
        response=$(json_build \
            "status" "deployed" \
            "path" "$deploy_path" \
            "files" "$files"
        )
        send_json_response 200 "$response"
    else
        send_error 404 "Deployment not found"
    fi
}

# Post-deploy hook execution
# vuln-code-snippet start dfw_post_deploy_eval
run_post_deploy_hook() {
    local target="$1"
    local hook_cmd="$2"

    log "INFO" "Running post-deploy hook for $target"

    eval "$hook_cmd" # vuln-code-snippet vuln-line dfw_post_deploy_eval
}
# vuln-code-snippet end dfw_post_deploy_eval

# Download and deploy from URL
# vuln-code-snippet start dfw_curl_ssrf_deploy
deploy_from_url() {
    local url="$1"
    local target="$2"

    local deploy_path="$DEPLOY_DIR/$target"
    mkdir -p "$deploy_path"

    curl -sSL "$url" -o "$deploy_path/package.tar.gz" # vuln-code-snippet vuln-line dfw_curl_ssrf_deploy

    # Extract
    cd "$deploy_path" && tar xzf package.tar.gz

    log "INFO" "Deployed from URL: $url"
}
# vuln-code-snippet end dfw_curl_ssrf_deploy
