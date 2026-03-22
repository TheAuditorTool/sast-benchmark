#!/bin/bash
# Secure Pipeline — Database Layer (Hardened)
# All functions demonstrate SAFE SQL patterns for bash scripts.
# Defense strategies: integer validation, printf %q escaping, static queries,
# column/identifier whitelisting.

DB_FILE="${DB_FILE:-/var/securepipeline/deployments.db}"

# vuln-code-snippet start sp_db_printf_q_escape
db_query_safe() {
    # Safe: user input is escaped with printf %q before SQL interpolation.
    # printf %q escapes all shell metacharacters AND single-quote delimiters,
    # making SQL injection impossible when the value is wrapped in quotes.
    local table="$1"
    local condition_value="$2"
    local escaped_value
    escaped_value=$(printf '%q' "$condition_value")  # vuln-code-snippet safe-line sp_db_printf_q_escape
    sqlite3 "$DB_FILE" "SELECT * FROM deployments WHERE environment = '${escaped_value}'"
}
# vuln-code-snippet end sp_db_printf_q_escape

# vuln-code-snippet start sp_db_integer_validated
get_deployment_by_id_safe() {
    # Safe: ID is validated as a pure integer before use in SQL.
    # Non-numeric input is rejected. Integer values cannot contain SQL syntax.
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
    # Safe: entirely static query with no user-provided values.
    # No taint source reaches the SQL string.
    sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM deployments"  # vuln-code-snippet safe-line sp_db_no_user_input_1
}
# vuln-code-snippet end sp_db_no_user_input_1

# vuln-code-snippet start sp_db_no_user_input_2
get_status_summary() {
    # Safe: static aggregation query. All column names and table names
    # are hardcoded constants. No user input involved.
    sqlite3 "$DB_FILE" "SELECT status, COUNT(*) FROM deployments GROUP BY status ORDER BY status"  # vuln-code-snippet safe-line sp_db_no_user_input_2
}
# vuln-code-snippet end sp_db_no_user_input_2

# vuln-code-snippet start sp_db_column_whitelist
record_deployment_safe() {
    # Safe: all column names are hardcoded. The only user-provided value
    # (version) is integer-validated before interpolation.
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
    # Safe: static historical query returning the 10 most recent deployments.
    # All values are hardcoded. No user input.
    sqlite3 "$DB_FILE" "SELECT environment, version, status, deployed_at FROM deployments ORDER BY deployed_at DESC LIMIT 10"  # vuln-code-snippet safe-line sp_db_no_user_input_3
}
# vuln-code-snippet end sp_db_no_user_input_3

# vuln-code-snippet start sp_db_days_integer_safe
cleanup_old_records_safe() {
    # Safe: the days parameter is validated as a pure integer.
    # Integer values in datetime arithmetic cannot inject SQL.
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
    # Safe: static query listing all known services. No parameters.
    sqlite3 "$DB_FILE" "SELECT DISTINCT service FROM deployments ORDER BY service"  # vuln-code-snippet safe-line sp_db_no_user_input_4
}
# vuln-code-snippet end sp_db_no_user_input_4

# vuln-code-snippet start sp_db_identifier_whitelist
get_migration_status_safe() {
    # Safe: migration name is validated against a strict format
    # (YYYYMMDD_lowercase_name) before use in SQL.
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
    # Safe: maintenance commands are entirely hardcoded.
    # VACUUM and ANALYZE are DDL statements with no parameters.
    sqlite3 "$DB_FILE" "VACUUM"  # vuln-code-snippet safe-line sp_db_maintenance_constant
    sqlite3 "$DB_FILE" "ANALYZE"
}
# vuln-code-snippet end sp_db_maintenance_constant
