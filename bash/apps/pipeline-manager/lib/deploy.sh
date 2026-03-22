#!/bin/bash
# Deployment operations for Pipeline Manager
# Handles deployments, rollbacks, and service management

# ============================================================================
# Deployment Execution
# ============================================================================
execute_deployment() {
    local environment="$1"
    local version="$2"

    log_info "Executing deployment: ${environment} -> ${version}"

    # Get deployment configuration
    local deploy_host
    deploy_host=$(get_env_config "${environment}" "deploy_server")

    local deploy_user
    deploy_user=$(get_env_config "${environment}" "deploy_user")

    # Pull latest code
    pull_code "${version}"

    # Build if needed
    if [[ -f "Makefile" ]] || [[ -f "build.sh" ]]; then
        run_build "${version}"
    fi

    # Deploy to target
    deploy_to_target "${environment}" "${deploy_host}" "${deploy_user}" "${version}"

    # Verify deployment
    verify_deployment "${environment}"
}

execute_rollback() {
    local environment="$1"
    local version="$2"

    log_info "Executing rollback: ${environment} -> ${version}"

    # Deploy the previous version
    deploy_to_target "${environment}" "${DEPLOY_SERVER}" "${DEPLOY_USER}" "${version}"

    # Record rollback
    record_deployment "${environment}" "${version}" "" "ROLLBACK"

    # Verify rollback
    verify_deployment "${environment}"
}

# ============================================================================
# Git Operations
# ============================================================================
pull_code() {
    local version="$1"

    log_info "Pulling code for version: ${version}"

    if [[ "${version}" == "latest" ]] || [[ "${version}" == "HEAD" ]]; then
        git pull origin main
    else
        git fetch --tags
        git checkout "${version}"
    fi
}

get_current_commit() {
    git rev-parse HEAD
}

get_commit_message() {
    local sha="${1:-HEAD}"
    git log -1 --format="%s" "${sha}"
}

# ============================================================================
# Build Process
# ============================================================================
run_build() {
    local version="$1"

    log_info "Running build for ${version}"

    if [[ -f "Makefile" ]]; then
        make clean
        make build
    elif [[ -f "build.sh" ]]; then
        ./build.sh "${version}"
    fi
}

# ============================================================================
# Remote Deployment
# ============================================================================
deploy_to_target() {
    local environment="$1"
    local host="$2"
    local user="$3"
    local version="$4"

    log_info "Deploying to ${user}@${host}"

    local ssh_opts="-o BatchMode=yes -o ConnectTimeout=30"

    # vuln-code-snippet start ssh_host_key_bypass
    if [[ "${DEPLOY_SKIP_HOST_CHECK:-false}" == "true" ]]; then
        ssh_opts="${ssh_opts} -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null"  # vuln-code-snippet vuln-line ssh_host_key_bypass
    fi
    # vuln-code-snippet end ssh_host_key_bypass

    local deploy_key="${DEPLOY_KEY/#\~/$HOME}"
    if [[ -f "${deploy_key}" ]]; then
        ssh_opts="${ssh_opts} -i ${deploy_key}"
    fi

    # Create deployment package
    local package_path
    package_path=$(create_deploy_package "${version}")

    # Copy package to remote
    scp ${ssh_opts} "${package_path}" "${user}@${host}:/tmp/deploy_${version}.tar.gz"

    # Execute remote deployment script
    # vuln-code-snippet start ssh_command_injection
    local deploy_cmd="cd /opt/app && tar xzf /tmp/deploy_${version}.tar.gz && ./deploy-hook.sh ${version}"
    ssh ${ssh_opts} "${user}@${host}" "${deploy_cmd}"  # vuln-code-snippet vuln-line ssh_command_injection
    # vuln-code-snippet end ssh_command_injection

    # Cleanup
    rm -f "${package_path}"

    log_info "Deployment package installed on ${host}"
}

create_deploy_package() {
    local version="$1"

    local package_dir
    package_dir=$(mktemp -d)

    local package_path="${package_dir}/deploy_${version}.tar.gz"

    # Create tarball excluding sensitive files
    tar czf "${package_path}" \
        --exclude='.git' \
        --exclude='*.log' \
        --exclude='node_modules' \
        --exclude='.env*' \
        .

    echo "${package_path}"
}

# ============================================================================
# Deployment Hooks
# ============================================================================
run_deploy_hooks() {
    local phase="$1"
    local environment="$2"

    local hooks_dir="${SCRIPT_DIR}/hooks/${phase}"

    if [[ ! -d "${hooks_dir}" ]]; then
        log_debug "No ${phase} hooks directory found"
        return 0
    fi

    log_info "Running ${phase}-deployment hooks"

    for hook in "${hooks_dir}"/*.sh; do
        if [[ -x "${hook}" ]]; then
            log_debug "Executing hook: ${hook}"
            "${hook}" "${environment}"
        fi
    done
}

# vuln-code-snippet start custom_hook_execution
run_custom_hook() {
    local hook_name="$1"
    shift
    local hook_args=("$@")

    local hook_cmd="${SCRIPT_DIR}/hooks/custom/${hook_name}"

    if [[ -x "${hook_cmd}" ]]; then
        # Execute hook with arguments
        $hook_cmd "${hook_args[@]}"  # vuln-code-snippet vuln-line custom_hook_execution
    else
        log_warn "Custom hook not found or not executable: ${hook_name}"
    fi
}
# vuln-code-snippet end custom_hook_execution

# ============================================================================
# Deployment Verification
# ============================================================================
verify_deployment() {
    local environment="$1"

    log_info "Verifying deployment for ${environment}"

    # Get health check endpoint
    local health_endpoint
    health_endpoint=$(get_env_config "${environment}" "health_endpoint")

    if [[ -z "${health_endpoint}" ]]; then
        health_endpoint="${API_ENDPOINT}/health"
    fi

    # Check health endpoint
    local max_attempts=30
    local attempt=1

    while [[ ${attempt} -le ${max_attempts} ]]; do
        log_debug "Health check attempt ${attempt}/${max_attempts}"

        local response
        response=$(curl -sf "${health_endpoint}" 2>/dev/null || true)

        if [[ -n "${response}" ]]; then
            log_info "Deployment verified: health check passed"
            return 0
        fi

        sleep 2
        ((attempt++))
    done

    log_error "Deployment verification failed: health check timeout"
    return 1
}

# ============================================================================
# Service Management
# ============================================================================
# vuln-code-snippet start ssh_sudo_command_injection
restart_service() {
    local service="$1"
    local host="${2:-localhost}"

    log_info "Restarting service: ${service} on ${host}"

    if [[ "${host}" == "localhost" ]]; then
        # Local restart
        systemctl restart "${service}"
    else
        ssh "${DEPLOY_USER}@${host}" "sudo systemctl restart ${service}"  # vuln-code-snippet vuln-line ssh_sudo_command_injection
    fi
}
# vuln-code-snippet end ssh_sudo_command_injection

stop_service() {
    local service="$1"
    local host="${2:-localhost}"

    log_info "Stopping service: ${service}"

    if [[ "${host}" == "localhost" ]]; then
        systemctl stop "${service}"
    else
        ssh "${DEPLOY_USER}@${host}" "sudo systemctl stop ${service}"
    fi
}

start_service() {
    local service="$1"
    local host="${2:-localhost}"

    log_info "Starting service: ${service}"

    if [[ "${host}" == "localhost" ]]; then
        systemctl start "${service}"
    else
        ssh "${DEPLOY_USER}@${host}" "sudo systemctl start ${service}"
    fi
}

get_service_status() {
    local service="$1"
    local host="${2:-localhost}"

    if [[ "${host}" == "localhost" ]]; then
        systemctl status "${service}" --no-pager
    else
        ssh "${DEPLOY_USER}@${host}" "systemctl status ${service} --no-pager"
    fi
}

# ============================================================================
# Container Deployment (Docker)
# ============================================================================
# vuln-code-snippet start docker_unquoted_vars
deploy_container() {
    local image="$1"
    local container_name="$2"
    local environment="$3"

    log_info "Deploying container: ${container_name}"

    # Get environment-specific configuration
    local env_file="${SCRIPT_DIR}/config/${environment}.env"

    # Stop existing container
    docker stop "${container_name}" 2>/dev/null || true
    docker rm "${container_name}" 2>/dev/null || true

    # Pull new image
    docker pull "${image}"

    # Run new container
    docker run -d \
        --name ${container_name} \
        --env-file ${env_file} \
        --restart unless-stopped \
        ${image}  # vuln-code-snippet vuln-line docker_unquoted_vars

    # Wait for container to be healthy
    local max_wait=60
    local waited=0

    while [[ ${waited} -lt ${max_wait} ]]; do
        local health
        health=$(docker inspect --format='{{.State.Health.Status}}' "${container_name}" 2>/dev/null || echo "unknown")

        if [[ "${health}" == "healthy" ]]; then
            log_info "Container ${container_name} is healthy"
            return 0
        fi

        sleep 2
        ((waited += 2))
    done

    log_error "Container health check timeout"
    return 1
}
# vuln-code-snippet end docker_unquoted_vars

# ============================================================================
# Kubernetes Deployment
# ============================================================================
deploy_kubernetes() {
    local manifest="$1"
    local namespace="${2:-default}"

    log_info "Deploying to Kubernetes: ${manifest}"

    # Apply manifest
    kubectl apply -f "${manifest}" -n "${namespace}"

    # Wait for rollout
    local deployment_name
    deployment_name=$(grep "name:" "${manifest}" | head -1 | awk '{print $2}')

    kubectl rollout status deployment/"${deployment_name}" -n "${namespace}" --timeout=300s
}

# ============================================================================
# Rollback Support
# ============================================================================
create_rollback_point() {
    local environment="$1"

    log_info "Creating rollback point for ${environment}"

    local current_version
    current_version=$(get_deployment_status "${environment}" | awk 'NR==2 {print $2}')

    if [[ -n "${current_version}" ]]; then
        echo "${current_version}" > "${DATA_DIR}/rollback_${environment}.txt"
        log_info "Rollback point saved: ${current_version}"
    fi
}

get_rollback_version() {
    local environment="$1"

    local rollback_file="${DATA_DIR}/rollback_${environment}.txt"

    if [[ -f "${rollback_file}" ]]; then
        cat "${rollback_file}"
    else
        # Fall back to database
        get_previous_version "${environment}" 1
    fi
}

# vuln-code-snippet start chmod777_deploy_dir
prepare_deploy_directory() {
    local deploy_path="$1"

    mkdir -p "${deploy_path}"

    chmod 777 "${deploy_path}"  # vuln-code-snippet vuln-line chmod777_deploy_dir
}
# vuln-code-snippet end chmod777_deploy_dir

# Safe version
# vuln-code-snippet start chmod755_deploy_dir
prepare_deploy_directory() {
    local deploy_path="$1"

    mkdir -p "${deploy_path}"
    chmod 755 "${deploy_path}"  # vuln-code-snippet safe-line chmod755_deploy_dir
}
# vuln-code-snippet end chmod755_deploy_dir
