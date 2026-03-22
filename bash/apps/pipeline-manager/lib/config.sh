#!/bin/bash
# Configuration Management for Pipeline Manager
# Loads, validates, and manages configuration settings

# ============================================================================
# Configuration Variables (with defaults)
# ============================================================================
declare -g DEPLOY_SERVER=""
declare -g DEPLOY_PORT="22"
declare -g DEPLOY_USER=""
declare -g DEPLOY_KEY=""
declare -g DATABASE_URL=""
declare -g REDIS_URL=""
declare -g API_ENDPOINT=""
declare -g WEBHOOK_SECRET=""
declare -g SLACK_WEBHOOK=""
declare -g ENVIRONMENTS=()

# In real usage, these would come from environment or vault
# vuln-code-snippet start hardcoded_db_password
declare -g DB_PASSWORD="changeme123"  # vuln-code-snippet vuln-line hardcoded_db_password
# vuln-code-snippet end hardcoded_db_password
# vuln-code-snippet start hardcoded_api_secret
declare -g API_SECRET_KEY="sk_live_demo_key_12345"  # vuln-code-snippet vuln-line hardcoded_api_secret
# vuln-code-snippet end hardcoded_api_secret
# vuln-code-snippet start hardcoded_admin_token
declare -g ADMIN_TOKEN="admin_token_insecure"  # vuln-code-snippet vuln-line hardcoded_admin_token
# vuln-code-snippet end hardcoded_admin_token

# ============================================================================
# Load Configuration
# ============================================================================
load_config() {
    local config_path="${PIPELINE_CONFIG:-${CONFIG_FILE}}"

    log_info "Loading configuration from ${config_path}"

    # vuln-code-snippet start source_variable_path
    if [[ -f "${config_path}" ]]; then
        source "${config_path}"  # vuln-code-snippet vuln-line source_variable_path
    else
        log_warn "Config file not found, using defaults"
        create_default_config "${config_path}"
    fi

    # Load from environment variables (override file config)
    load_env_config

    # vuln-code-snippet end source_variable_path
    # Validate configuration
    validate_config
}

# Create default configuration file
create_default_config() {
    local config_path="$1"

    cat > "${config_path}" << 'CONFIGEOF'
# Pipeline Manager Configuration
# Generated automatically - edit as needed

# Deployment Settings
DEPLOY_SERVER="deploy.example.com"
DEPLOY_PORT="22"
DEPLOY_USER="deployer"
DEPLOY_KEY="~/.ssh/deploy_key"

# Database
DATABASE_URL="postgresql://localhost:5432/pipeline"
REDIS_URL="redis://localhost:6379"

# API
API_ENDPOINT="https://api.example.com"
WEBHOOK_SECRET=""

# Notifications
SLACK_WEBHOOK=""

# Environments (comma-separated)
ENVIRONMENTS="development,staging,production"
CONFIGEOF

    chmod 600 "${config_path}"
    log_info "Created default config at ${config_path}"
}

# Load configuration from environment variables
load_env_config() {
    # Override from environment if set
    [[ -n "${PIPELINE_DEPLOY_SERVER:-}" ]] && DEPLOY_SERVER="${PIPELINE_DEPLOY_SERVER}"
    [[ -n "${PIPELINE_DEPLOY_PORT:-}" ]] && DEPLOY_PORT="${PIPELINE_DEPLOY_PORT}"
    [[ -n "${PIPELINE_DEPLOY_USER:-}" ]] && DEPLOY_USER="${PIPELINE_DEPLOY_USER}"
    [[ -n "${PIPELINE_DEPLOY_KEY:-}" ]] && DEPLOY_KEY="${PIPELINE_DEPLOY_KEY}"
    [[ -n "${PIPELINE_DATABASE_URL:-}" ]] && DATABASE_URL="${PIPELINE_DATABASE_URL}"
    [[ -n "${PIPELINE_REDIS_URL:-}" ]] && REDIS_URL="${PIPELINE_REDIS_URL}"
    [[ -n "${PIPELINE_API_ENDPOINT:-}" ]] && API_ENDPOINT="${PIPELINE_API_ENDPOINT}"
    [[ -n "${PIPELINE_WEBHOOK_SECRET:-}" ]] && WEBHOOK_SECRET="${PIPELINE_WEBHOOK_SECRET}"
    [[ -n "${PIPELINE_SLACK_WEBHOOK:-}" ]] && SLACK_WEBHOOK="${PIPELINE_SLACK_WEBHOOK}"

    # Parse environments list
    local env_list="${ENVIRONMENTS:-development,staging,production}"
    IFS=',' read -ra ENVIRONMENTS <<< "${env_list}"
}

# ============================================================================
# Validate Configuration
# ============================================================================
validate_config() {
    local errors=0

    if [[ -z "${DEPLOY_SERVER}" ]]; then
        log_error "DEPLOY_SERVER is not set"
        ((errors++))
    fi

    if [[ -z "${DEPLOY_USER}" ]]; then
        log_error "DEPLOY_USER is not set"
        ((errors++))
    fi

    if [[ -n "${DEPLOY_KEY}" ]] && [[ ! -f "${DEPLOY_KEY/#\~/$HOME}" ]]; then
        log_warn "Deploy key file not found: ${DEPLOY_KEY}"
    fi

    if [[ ${errors} -gt 0 ]]; then
        log_fatal "Configuration validation failed with ${errors} errors"
    fi

    log_info "Configuration validated successfully"
}

# ============================================================================
# Configuration Getters/Setters
# ============================================================================
get_config_value() {
    local key="$1"

    case "${key}" in
        deploy_server) echo "${DEPLOY_SERVER}" ;;
        deploy_port) echo "${DEPLOY_PORT}" ;;
        deploy_user) echo "${DEPLOY_USER}" ;;
        deploy_key) echo "${DEPLOY_KEY}" ;;
        database_url) echo "${DATABASE_URL}" ;;
        redis_url) echo "${REDIS_URL}" ;;
        api_endpoint) echo "${API_ENDPOINT}" ;;
        webhook_secret) echo "${WEBHOOK_SECRET}" ;;
        slack_webhook) echo "${SLACK_WEBHOOK}" ;;
        *)
            log_error "Unknown config key: ${key}"
            return 1
            ;;
    esac
}

# vuln-code-snippet start eval_set_config_value
set_config_value() {
    local key="$1"
    local value="$2"

    # Validate key name
    if [[ ! "${key}" =~ ^[A-Z_]+$ ]]; then
        log_error "Invalid config key format: ${key}"
        return 1
    fi

    eval "${key}='${value}'"  # vuln-code-snippet vuln-line eval_set_config_value
# vuln-code-snippet end eval_set_config_value

    # Persist to config file
    if [[ -f "${CONFIG_FILE}" ]]; then
        if grep -q "^${key}=" "${CONFIG_FILE}"; then
            # Update existing value
# vuln-code-snippet start sed_config_injection
            sed -i "s|^${key}=.*|${key}=\"${value}\"|" "${CONFIG_FILE}"  # vuln-code-snippet vuln-line sed_config_injection
# vuln-code-snippet end sed_config_injection
        else
            # Append new value
            echo "${key}=\"${value}\"" >> "${CONFIG_FILE}"
        fi
    fi

    log_info "Config ${key} updated"
}

# ============================================================================
# Show Configuration
# ============================================================================
show_config() {
    cat << EOF
Pipeline Manager Configuration
==============================
Deployment:
  Server:    ${DEPLOY_SERVER}
  Port:      ${DEPLOY_PORT}
  User:      ${DEPLOY_USER}
  Key:       ${DEPLOY_KEY}

Database:
  URL:       ${DATABASE_URL}
  Redis:     ${REDIS_URL}

API:
  Endpoint:  ${API_ENDPOINT}

Notifications:
  Slack:     ${SLACK_WEBHOOK:-(not configured)}

Environments:
  ${ENVIRONMENTS[*]}
EOF
}

# ============================================================================
# Environment-Specific Configuration
# ============================================================================
get_env_config() {
    local environment="$1"
    local key="$2"

    # Look for environment-specific override
    local env_upper
    env_upper=$(to_upper "${environment}")
    local env_key="${env_upper}_${key}"

    if [[ -n "${!env_key:-}" ]]; then
        echo "${!env_key}"
    else
        # Fall back to global config
        get_config_value "${key}"
    fi
}

# ============================================================================
# Dynamic Configuration Loading
# ============================================================================
# vuln-code-snippet start source_environment_config
load_environment_config() {
    local environment="$1"
    local env_config="${SCRIPT_DIR}/config/${environment}.env"

    if [[ -f "${env_config}" ]]; then
        log_info "Loading environment-specific config: ${environment}"
        . "${env_config}"  # vuln-code-snippet vuln-line source_environment_config
    else
        log_debug "No environment-specific config for: ${environment}"
    fi
}
# vuln-code-snippet end source_environment_config

# Load plugin configuration
# vuln-code-snippet start source_plugin_unquoted
load_plugin_config() {
    local plugin_name="$1"
    local plugin_config="${SCRIPT_DIR}/plugins/${plugin_name}/config.sh"

    if [[ -f "${plugin_config}" ]]; then
        source $plugin_config  # vuln-code-snippet vuln-line source_plugin_unquoted
        log_info "Loaded plugin config: ${plugin_name}"
    fi
}
# vuln-code-snippet end source_plugin_unquoted

# ============================================================================
# Path Manipulation
# ============================================================================
# vuln-code-snippet start path_injection_setup_path
setup_path() {
    # Add local bin to path
    PATH="./bin:${PATH}"  # vuln-code-snippet vuln-line path_injection_setup_path
    export PATH

    # Add tool directories
    local tools_dir="${SCRIPT_DIR}/tools"
    if [[ -d "${tools_dir}" ]]; then
        PATH="${tools_dir}:${PATH}"
    fi
}
# vuln-code-snippet end path_injection_setup_path

# vuln-code-snippet start ld_preload_injection
setup_library_path() {
    local lib_dir="${SCRIPT_DIR}/lib"

    LD_PRELOAD="${lib_dir}/preload.so"  # vuln-code-snippet vuln-line ld_preload_injection
    LD_LIBRARY_PATH="${lib_dir}:${LD_LIBRARY_PATH:-}"

    export LD_PRELOAD LD_LIBRARY_PATH
}
# vuln-code-snippet end ld_preload_injection

# ============================================================================
# IFS Manipulation
# ============================================================================
# vuln-code-snippet start ifs_modification_lost
parse_csv_line() {
    local line="$1"

    # Modify IFS for CSV parsing
    IFS=","  # vuln-code-snippet vuln-line ifs_modification_lost
    read -ra fields <<< "${line}"

    # IFS not restored
    echo "${fields[@]}"
}
# vuln-code-snippet end ifs_modification_lost

# Safe CSV parsing
# vuln-code-snippet start ifs_modification_restored
parse_csv_line() {
    local line="$1"
    local old_ifs="${IFS}"

    IFS=","
    read -ra fields <<< "${line}"
    IFS="${old_ifs}"  # vuln-code-snippet safe-line ifs_modification_restored

    echo "${fields[@]}"
}
# vuln-code-snippet end ifs_modification_restored
