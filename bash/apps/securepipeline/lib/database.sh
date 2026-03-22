#!/bin/bash
# Secure Pipeline — Database Layer
# Integer validation, printf %q escaping, static queries,
# column/identifier whitelisting.

DB_FILE="${DB_FILE:-/var/securepipeline/deployments.db}"

# vuln-code-snippet start sp_db_printf_q_escape
db_query_safe() {
    # User input is escaped with printf %q before SQL interpolation.
    local table="$1"
    local condition_value="$2"
    local escaped_value
    escaped_value=$(printf '%q' "$condition_value")  # vuln-code-snippet safe-line sp_db_printf_q_escape
    sqlite3 "$DB_FILE" "SELECT * FROM deployments WHERE environment = '${escaped_value}'"
}
# vuln-code-snippet end sp_db_printf_q_escape

# vuln-code-snippet start sp_db_integer_validated
get_deployment_by_id_safe() {
    # ID is validated as a pure integer before use in SQL.
    local id="$1"

    if [[ ! "$id" =~ ^[0-9]+$ ]]; then
        echo "Invalid ID: must be integer" >&2
        return 1
    fi

    sqlite3 "$DB_FILE" "SELECT * FROM deployments WHERE id = ${id}"  # vuln-code-snippet safe-line sp_db_integer_validated
}
# vuln-code-snippet end sp_db_integer_validated

# vuln-code-snippet start sp_db_no_user_input_1
get_deployment_count() {
    # Entirely static query with no user-provided values.
    sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM deployments"  # vuln-code-snippet safe-line sp_db_no_user_input_1
}
# vuln-code-snippet end sp_db_no_user_input_1

# vuln-code-snippet start sp_db_no_user_input_2
get_status_summary() {
    # Static aggregation query. All column and table names are constants.
    sqlite3 "$DB_FILE" "SELECT status, COUNT(*) FROM deployments GROUP BY status ORDER BY status"  # vuln-code-snippet safe-line sp_db_no_user_input_2
}
# vuln-code-snippet end sp_db_no_user_input_2

# vuln-code-snippet start sp_db_column_whitelist
record_deployment_safe() {
    # All column names are hardcoded. Version is integer-validated.
    local environment="$1"
    local version="$2"

    if [[ ! "$version" =~ ^[0-9]+$ ]]; then
        echo "Invalid version: must be integer" >&2
        return 1
    fi

    local timestamp
    timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    sqlite3 "$DB_FILE" "INSERT INTO deployments (environment, version, deployed_at) VALUES ('production', ${version}, '${timestamp}')"  # vuln-code-snippet safe-line sp_db_column_whitelist
}
# vuln-code-snippet end sp_db_column_whitelist

# vuln-code-snippet start sp_db_no_user_input_3
get_recent_deployments() {
    # Static historical query returning the 10 most recent deployments.
    sqlite3 "$DB_FILE" "SELECT environment, version, status, deployed_at FROM deployments ORDER BY deployed_at DESC LIMIT 10"  # vuln-code-snippet safe-line sp_db_no_user_input_3
}
# vuln-code-snippet end sp_db_no_user_input_3

# vuln-code-snippet start sp_db_days_integer_safe
cleanup_old_records_safe() {
    # Days parameter is validated as a pure integer.
    local days="$1"

    if [[ ! "$days" =~ ^[0-9]+$ ]]; then
        echo "Invalid days: must be integer" >&2
        return 1
    fi

    sqlite3 "$DB_FILE" "DELETE FROM deployments WHERE deployed_at < datetime('now', '-${days} days')"  # vuln-code-snippet safe-line sp_db_days_integer_safe
}
# vuln-code-snippet end sp_db_days_integer_safe

# vuln-code-snippet start sp_db_no_user_input_4
list_distinct_services() {
    # Static query listing all known services.
    sqlite3 "$DB_FILE" "SELECT DISTINCT service FROM deployments ORDER BY service"  # vuln-code-snippet safe-line sp_db_no_user_input_4
}
# vuln-code-snippet end sp_db_no_user_input_4

# vuln-code-snippet start sp_db_identifier_whitelist
get_migration_status_safe() {
    # Migration name validated against strict format (YYYYMMDD_lowercase_name).
    local migration_name="$1"

    if [[ ! "$migration_name" =~ ^[0-9]{8}_[a-z_]+$ ]]; then
        echo "Invalid migration name format" >&2
        return 1
    fi

    sqlite3 "$DB_FILE" "SELECT status FROM migrations WHERE name = '${migration_name}'"  # vuln-code-snippet safe-line sp_db_identifier_whitelist
}
# vuln-code-snippet end sp_db_identifier_whitelist

# vuln-code-snippet start sp_db_maintenance_constant
run_db_maintenance() {
    # Maintenance commands are entirely hardcoded.
    sqlite3 "$DB_FILE" "VACUUM"  # vuln-code-snippet safe-line sp_db_maintenance_constant
    sqlite3 "$DB_FILE" "ANALYZE"
}
# vuln-code-snippet end sp_db_maintenance_constant
