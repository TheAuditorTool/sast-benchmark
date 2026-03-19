#!/bin/bash
# deploy.sh - Deployment script for DeployBot
#
# CONTAINS BOTH VULNERABLE AND SAFE CODE PATHS
# Environment variable SAFE_MODE controls which path is used
#
# Usage: ./deploy.sh <target> <environment> <branch>

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/common.sh"
source "$SCRIPT_DIR/../lib/validate.sh"

# Arguments from Python subprocess call
TARGET="$1"
ENVIRONMENT="$2"
BRANCH="${3:-main}"

DEPLOY_DIR="${DEPLOY_DIR:-/var/deployments}"

log_info "Starting deployment"
log_info "Target: $TARGET"
log_info "Environment: $ENVIRONMENT"
log_info "Branch: $BRANCH"

# Check if running in safe mode
# vuln-code-snippet start dfo_deploy_safe_validated
if [[ "${SAFE_MODE:-false}" == "true" ]]; then
    # === SAFE PATH ===
    log_info "Running in SAFE MODE"

    # Validate all inputs
    if ! validate_identifier "$TARGET"; then  # vuln-code-snippet safe-line dfo_deploy_safe_validated
        log_error "Target validation failed"
        exit 1
    fi

    if ! validate_environment "$ENVIRONMENT"; then
        log_error "Environment validation failed"
        exit 1
    fi

    if ! validate_branch "$BRANCH"; then
        log_error "Branch validation failed"
        exit 1
    fi

    # SAFE: Construct path and validate it
    DEPLOY_PATH="$DEPLOY_DIR/$TARGET"
    if ! validate_path "$DEPLOY_PATH" "$DEPLOY_DIR"; then
        log_error "Path validation failed"
        exit 1
    fi

    # SAFE: Use validated values
    log_info "Deploying to: $DEPLOY_PATH"
    mkdir -p "$DEPLOY_PATH"

    # SAFE: Git operations with validated branch
    cd "$DEPLOY_PATH"
    if [[ -d ".git" ]]; then
        git fetch origin
        git checkout "$BRANCH"
        git pull origin "$BRANCH"
    else
        # SAFE: Clone with validated inputs
        git clone --branch "$BRANCH" "https://github.com/org/$TARGET.git" .
    fi
# vuln-code-snippet end dfo_deploy_safe_validated

else
    # === VULNERABLE PATH ===
    log_warn "Running in UNSAFE MODE"

    # VULNERABLE: No input validation
    # TAINT FLOW: $TARGET -> file path (Path Traversal)
    DEPLOY_PATH="$DEPLOY_DIR/$TARGET"

    log_info "Deploying to: $DEPLOY_PATH"

    # VULNERABLE: mkdir with unsanitized path
    mkdir -p "$DEPLOY_PATH"

    # VULNERABLE: cd to unsanitized path
    cd "$DEPLOY_PATH"

    # VULNERABLE: Git operations with unsanitized branch
    # TAINT FLOW: $BRANCH -> git command (Command Injection via branch name)
    # vuln-code-snippet start dfo_deploy_git_branch_unquoted
    if [[ -d ".git" ]]; then
        git fetch origin
        git checkout $BRANCH  # VULNERABLE: Unquoted variable  # vuln-code-snippet vuln-line dfo_deploy_git_branch_unquoted
        git pull origin $BRANCH
    else
        # VULNERABLE: Unsanitized target in git URL
        # TAINT FLOW: $TARGET -> git clone URL (SSRF/Command Injection)
        git clone --branch $BRANCH "https://github.com/org/$TARGET.git" .
    fi
    # vuln-code-snippet end dfo_deploy_git_branch_unquoted

    # VULNERABLE: Execute arbitrary post-deploy script if specified
    # TAINT FLOW: $POST_DEPLOY_SCRIPT -> bash execution (Command Injection)
    # vuln-code-snippet start dfo_deploy_bash_c_hook
    if [[ -n "$POST_DEPLOY_SCRIPT" ]]; then
        log_info "Running post-deploy script: $POST_DEPLOY_SCRIPT"
        bash -c "$POST_DEPLOY_SCRIPT"  # vuln-code-snippet vuln-line dfo_deploy_bash_c_hook
    fi
    # vuln-code-snippet end dfo_deploy_bash_c_hook
fi

# Run post-deploy hooks
if [[ -f "$DEPLOY_PATH/hooks/post-deploy.sh" ]]; then
    log_info "Running post-deploy hooks"
    bash "$DEPLOY_PATH/hooks/post-deploy.sh"
fi

log_info "Deployment completed successfully"
exit 0
