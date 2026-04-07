#!/bin/bash
# Secure Pipeline — Configuration Management

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_DIR="${SCRIPT_DIR}/config"
PLUGIN_DIR="${SCRIPT_DIR}/plugins"

# vuln-code-snippet start sp_config_set_validated
set_config_value() {
    # Allowlist of known config keys. Value is printf-%q escaped.
    local key="$1"
    local value="$2"
    local config_file="${CONFIG_DIR}/app.conf"

    case "$key" in
        APP_MODE|APP_PORT|LOG_LEVEL|DB_HOST|DB_PORT|WORKERS)
            local escaped_value
            escaped_value=$(printf '%q' "$value")  # vuln-code-snippet safe-line sp_config_set_validated
            sed -i "s|^${key}=.*|${key}=${escaped_value}|" "$config_file"
            ;;
        *)
            echo "Unknown config key: $key" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end sp_config_set_validated

# vuln-code-snippet start sp_config_load_constant
load_config() {
    # Path assembled entirely from constants.
    local config_path="${CONFIG_DIR}/app.conf"

    if [[ -f "$config_path" ]]; then
        source "${CONFIG_DIR}/app.conf"  # vuln-code-snippet safe-line sp_config_load_constant
    else
        echo "Config not found: $config_path" >&2
        return 1
    fi
}
# vuln-code-snippet end sp_config_load_constant

# vuln-code-snippet start sp_config_env_validated
load_environment_config() {
    # Environment name validated against a fixed allowlist.
    local env_name="$1"

    case "$env_name" in
        dev|staging|prod)
            source "/etc/securepipeline/config/${env_name}.env"  # vuln-code-snippet safe-line sp_config_env_validated
            ;;
        *)
            echo "Invalid environment: $env_name" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end sp_config_env_validated

# vuln-code-snippet start sp_config_ifs_restored
parse_csv_line() {
    # IFS is saved before modification and restored after.
    local line="$1"
    local saved_ifs="$IFS"
    local -a fields

    IFS=','
    read -r -a fields <<< "$line"  # vuln-code-snippet safe-line sp_config_ifs_restored
    IFS="$saved_ifs"

    printf '%s\n' "${fields[@]}"
}
# vuln-code-snippet end sp_config_ifs_restored

# vuln-code-snippet start sp_config_absolute_path
setup_path() {
    # PATH is set to absolute system directories only.
    PATH="/usr/local/bin:/usr/bin:/bin"  # vuln-code-snippet safe-line sp_config_absolute_path
    export PATH
    hash -r
}
# vuln-code-snippet end sp_config_absolute_path

# vuln-code-snippet start sp_config_plugin_validated
load_plugin_config() {
    # Plugin name validated against a strict regex before path construction.
    local plugin_name="$1"

    if [[ ! "$plugin_name" =~ ^[a-z][a-z0-9_-]*$ ]]; then
        echo "Invalid plugin name: $plugin_name" >&2
        return 1
    fi

    local plugin_config="${PLUGIN_DIR}/${plugin_name}/config.sh"
    if [[ -f "$plugin_config" ]]; then
        source "$plugin_config"  # vuln-code-snippet safe-line sp_config_plugin_validated
    fi
}
# vuln-code-snippet end sp_config_plugin_validated

# vuln-code-snippet start sp_config_no_ld_preload
setup_library_path() {
    # Unset LD_PRELOAD and LD_LIBRARY_PATH. Set only approved library paths.
    unset LD_PRELOAD  # vuln-code-snippet safe-line sp_config_no_ld_preload
    unset LD_LIBRARY_PATH
    export LD_LIBRARY_PATH="/usr/lib:/usr/local/lib"
}
# vuln-code-snippet end sp_config_no_ld_preload

# vuln-code-snippet start sp_config_read_r_flag
read_config_value() {
    # read uses -r flag to prevent backslash interpretation.
    local config_file="$1"
    local target_key="$2"
    local key value

    while IFS='=' read -r key value; do  # vuln-code-snippet safe-line sp_config_read_r_flag
        if [[ "$key" == "$target_key" ]]; then
            echo "$value"
            return 0
        fi
    done < "$config_file"
    return 1
}
# vuln-code-snippet end sp_config_read_r_flag
