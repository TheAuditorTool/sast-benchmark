#!/bin/bash
# deploy.sh - Deployment script for DeployBot
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
# vuln-code-snippet start dfo_deploy_validated
if [[ "${SAFE_MODE:-false}" == "true" ]]; then
    log_info "Running in SAFE MODE"

    # Validate all inputs
    if ! validate_identifier "$TARGET"; then  # vuln-code-snippet safe-line dfo_deploy_validated
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

    DEPLOY_PATH="$DEPLOY_DIR/$TARGET"
    if ! validate_path "$DEPLOY_PATH" "$DEPLOY_DIR"; then
        log_error "Path validation failed"
        exit 1
    fi

    log_info "Deploying to: $DEPLOY_PATH"
    mkdir -p "$DEPLOY_PATH"

    cd "$DEPLOY_PATH"
    if [[ -d ".git" ]]; then
        git fetch origin
        git checkout "$BRANCH"
        git pull origin "$BRANCH"
    else
        git clone --branch "$BRANCH" "https://github.com/org/$TARGET.git" .
    fi
# vuln-code-snippet end dfo_deploy_validated

else
    log_warn "Running in UNSAFE MODE"

    DEPLOY_PATH="$DEPLOY_DIR/$TARGET"

    log_info "Deploying to: $DEPLOY_PATH"

    mkdir -p "$DEPLOY_PATH"

    cd "$DEPLOY_PATH"
    # vuln-code-snippet start dfo_deploy_git_branch_unquoted
    if [[ -d ".git" ]]; then
        git fetch origin
        git checkout $BRANCH  # vuln-code-snippet vuln-line dfo_deploy_git_branch_unquoted
        git pull origin $BRANCH
    else
        git clone --branch $BRANCH "https://github.com/org/$TARGET.git" .
    fi
    # vuln-code-snippet end dfo_deploy_git_branch_unquoted

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
