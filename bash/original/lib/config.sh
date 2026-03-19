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

# TRIGGERS: bash-hardcoded-credential (intentional demonstration)
# In real usage, these would come from environment or vault
# vuln-code-snippet start hardcodedDbPassword
declare -g DB_PASSWORD="changeme123"  # vuln-code-snippet vuln-line hardcodedDbPassword
# vuln-code-snippet end hardcodedDbPassword
# vuln-code-snippet start hardcodedApiSecret
declare -g API_SECRET_KEY="sk_live_demo_key_12345"  # vuln-code-snippet vuln-line hardcodedApiSecret
# vuln-code-snippet end hardcodedApiSecret
# vuln-code-snippet start hardcodedAdminToken
declare -g ADMIN_TOKEN="admin_token_insecure"  # vuln-code-snippet vuln-line hardcodedAdminToken
# vuln-code-snippet end hardcodedAdminToken

# ============================================================================
# Load Configuration
# ============================================================================
load_config() {
    local config_path="${PIPELINE_CONFIG:-${CONFIG_FILE}}"

    log_info "Loading configuration from ${config_path}"

    # vuln-code-snippet start sourceVariablePath
    if [[ -f "${config_path}" ]]; then
        # TRIGGERS: source with variable path
        source "${config_path}"  # vuln-code-snippet vuln-line sourceVariablePath
    else
        log_warn "Config file not found, using defaults"
        create_default_config "${config_path}"
    fi

    # Load from environment variables (override file config)
    load_env_config

    # vuln-code-snippet end sourceVariablePath
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

# TRIGGERS: eval with variable (intentional demonstration)
# vuln-code-snippet start evalSetConfigValue
set_config_value() {
    local key="$1"
    local value="$2"

    # Validate key name
    if [[ ! "${key}" =~ ^[A-Z_]+$ ]]; then
        log_error "Invalid config key format: ${key}"
        return 1
    fi

    # DANGEROUS: eval with user input - for demonstration
    eval "${key}='${value}'"  # vuln-code-snippet vuln-line evalSetConfigValue

    # Persist to config file
    if [[ -f "${CONFIG_FILE}" ]]; then
        if grep -q "^${key}=" "${CONFIG_FILE}"; then
            # Update existing value
            # vuln-code-snippet start sedConfigInjection
            sed -i "s|^${key}=.*|${key}=\"${value}\"|" "${CONFIG_FILE}"  # vuln-code-snippet vuln-line sedConfigInjection
            # vuln-code-snippet end sedConfigInjection
        else
            # Append new value
            echo "${key}=\"${value}\"" >> "${CONFIG_FILE}"
        fi
    fi

    log_info "Config ${key} updated"
}
# vuln-code-snippet end evalSetConfigValue

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

    # TRIGGERS: indirect expansion (intentional)
    if [[ -n "${!env_key:-}" ]]; then
        echo "${!env_key}"
    else
        # Fall back to global config
        get_config_value "${key}"
    fi
}

# ============================================================================
# Dynamic Configuration Loading
# TRIGGERS: bash-source-injection (with variable path - intentional)
# ============================================================================
# vuln-code-snippet start sourceEnvironmentConfig
load_environment_config() {
    local environment="$1"
    local env_config="${SCRIPT_DIR}/config/${environment}.env"

    if [[ -f "${env_config}" ]]; then
        log_info "Loading environment-specific config: ${environment}"
        . "${env_config}"  # vuln-code-snippet vuln-line sourceEnvironmentConfig
    else
        log_debug "No environment-specific config for: ${environment}"
    fi
}
# vuln-code-snippet end sourceEnvironmentConfig

# Load plugin configuration
# vuln-code-snippet start sourcePluginUnquoted
load_plugin_config() {
    local plugin_name="$1"
    local plugin_config="${SCRIPT_DIR}/plugins/${plugin_name}/config.sh"

    # TRIGGERS: source with variable (intentional)
    if [[ -f "${plugin_config}" ]]; then
        source $plugin_config  # vuln-code-snippet vuln-line sourcePluginUnquoted
        log_info "Loaded plugin config: ${plugin_name}"
    fi
}
# vuln-code-snippet end sourcePluginUnquoted

# ============================================================================
# Path Manipulation (Intentionally Vulnerable)
# TRIGGERS: bash-path-injection
# ============================================================================
# vuln-code-snippet start pathInjectionSetupPath
setup_path() {
    # Add local bin to path - BAD: relative path first
    PATH="./bin:${PATH}"  # vuln-code-snippet vuln-line pathInjectionSetupPath
    export PATH

    # Add tool directories
    local tools_dir="${SCRIPT_DIR}/tools"
    if [[ -d "${tools_dir}" ]]; then
        PATH="${tools_dir}:${PATH}"
    fi
}
# vuln-code-snippet end pathInjectionSetupPath

# TRIGGERS: bash-environment-injection (intentional)
# vuln-code-snippet start ldPreloadInjection
setup_library_path() {
    local lib_dir="${SCRIPT_DIR}/lib"

    # These are dangerous environment variables
    LD_PRELOAD="${lib_dir}/preload.so"  # vuln-code-snippet vuln-line ldPreloadInjection
    LD_LIBRARY_PATH="${lib_dir}:${LD_LIBRARY_PATH:-}"

    export LD_PRELOAD LD_LIBRARY_PATH
}
# vuln-code-snippet end ldPreloadInjection

# ============================================================================
# IFS Manipulation (Intentionally Vulnerable)
# TRIGGERS: bash-ifs-modified
# ============================================================================
# vuln-code-snippet start ifsModificationUnsafe
parse_csv_line() {
    local line="$1"

    # Modify IFS for CSV parsing
    IFS=","  # vuln-code-snippet vuln-line ifsModificationUnsafe
    read -ra fields <<< "${line}"

    # Should restore IFS but this is intentionally buggy
    echo "${fields[@]}"
}
# vuln-code-snippet end ifsModificationUnsafe

# Safe CSV parsing
# vuln-code-snippet start ifsModificationSafe
parse_csv_line_safe() {
    local line="$1"
    local old_ifs="${IFS}"

    IFS=","
    read -ra fields <<< "${line}"
    IFS="${old_ifs}"  # vuln-code-snippet safe-line ifsModificationSafe

    echo "${fields[@]}"
}
# vuln-code-snippet end ifsModificationSafe
