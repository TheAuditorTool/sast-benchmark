#!/bin/bash
# Database operations for Pipeline Manager
# SQLite-based deployment tracking and state management

# ============================================================================
# Database Initialization
# ============================================================================
init_database() {
    local db_path="${DB_FILE}"

    if [[ ! -f "${db_path}" ]]; then
        log_info "Initializing database at ${db_path}"
        create_database_schema "${db_path}"
    else
        log_debug "Database exists at ${db_path}"
    fi
}

create_database_schema() {
    local db_path="$1"

    # Ensure data directory exists
    mkdir -p "$(dirname "${db_path}")"

    sqlite3 "${db_path}" << 'SCHEMA'
-- Deployments table
CREATE TABLE IF NOT EXISTS deployments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    environment TEXT NOT NULL,
    version TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    deployed_by TEXT,
    commit_sha TEXT,
    notes TEXT
);

-- Locks table for deployment coordination
CREATE TABLE IF NOT EXISTS deploy_locks (
    environment TEXT PRIMARY KEY,
    locked_by TEXT NOT NULL,
    locked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME
);

-- Migrations table
CREATE TABLE IF NOT EXISTS migrations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    migration_id TEXT NOT NULL UNIQUE,
    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    checksum TEXT
);

-- Configuration history
CREATE TABLE IF NOT EXISTS config_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL,
    old_value TEXT,
    new_value TEXT,
    changed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    changed_by TEXT
);

-- Health checks
CREATE TABLE IF NOT EXISTS health_checks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    service TEXT NOT NULL,
    status TEXT NOT NULL,
    response_time_ms INTEGER,
    checked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    details TEXT
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_deployments_env ON deployments(environment);
CREATE INDEX IF NOT EXISTS idx_deployments_status ON deployments(status);
CREATE INDEX IF NOT EXISTS idx_health_checks_service ON health_checks(service);
SCHEMA

    log_info "Database schema created"
}

# ============================================================================
# Database Queries (Using sqlite3 command for real functionality)
# ============================================================================
db_query() {
    local query="$1"
    sqlite3 -header -column "${DB_FILE}" "${query}"
}

db_execute() {
    local query="$1"
    sqlite3 "${DB_FILE}" "${query}"
}

# vuln-code-snippet start sqlInjectionDbQueryUnsafe
# TRIGGERS: bash injection via SQL - user input not escaped (intentional)
db_query_unsafe() {
    local table="$1"
    local condition="$2"

    # DANGEROUS: Direct string interpolation
    sqlite3 "${DB_FILE}" "SELECT * FROM ${table} WHERE ${condition}" # vuln-code-snippet vuln-line sqlInjectionDbQueryUnsafe
}
# vuln-code-snippet end sqlInjectionDbQueryUnsafe

# vuln-code-snippet start dbQuerySafeEscaped
# Safe parameterized query
db_query_safe() {
    local query="$1"
    shift
    local params=("$@")

    # Use printf %q for escaping
    local escaped_query
    escaped_query=$(printf '%q' "${query}")

    sqlite3 "${DB_FILE}" "${escaped_query}" # vuln-code-snippet safe-line dbQuerySafeEscaped
}
# vuln-code-snippet end dbQuerySafeEscaped

# ============================================================================
# Deployment Records
# ============================================================================
# vuln-code-snippet start sqlInjectionRecordDeployment
record_deployment() {
    local environment="$1"
    local version="$2"
    local commit_sha="${3:-}"
    local notes="${4:-}"
    local deployed_by="${USER:-unknown}"

    db_execute "
        INSERT INTO deployments (environment, version, status, deployed_by, commit_sha, notes)
        VALUES ('${environment}', '${version}', 'completed', '${deployed_by}', '${commit_sha}', '${notes}') # vuln-code-snippet vuln-line sqlInjectionRecordDeployment
    "

    log_info "Recorded deployment: ${environment} -> ${version}"
}
# vuln-code-snippet end sqlInjectionRecordDeployment

# vuln-code-snippet start sqlInjectionGetDeploymentStatus
get_deployment_status() {
    local environment="$1"

    db_query " # vuln-code-snippet vuln-line sqlInjectionGetDeploymentStatus
        SELECT id, version, status, started_at, completed_at, deployed_by
        FROM deployments
        WHERE environment = '${environment}'
        ORDER BY id DESC
        LIMIT 1
    "
}
# vuln-code-snippet end sqlInjectionGetDeploymentStatus

# vuln-code-snippet start getAllDeploymentStatusNoInput
get_all_deployment_status() {
    db_query " # vuln-code-snippet safe-line getAllDeploymentStatusNoInput
        SELECT environment, version, status, completed_at
        FROM deployments
        WHERE id IN (
            SELECT MAX(id)
            FROM deployments
            GROUP BY environment
        )
        ORDER BY environment
    "
}
# vuln-code-snippet end getAllDeploymentStatusNoInput

# vuln-code-snippet start sqlInjectionGetPreviousVersion
get_previous_version() {
    local environment="$1"
    local steps="${2:-1}"

    # Calculate offset
    local offset=$((steps))

    local result
    result=$(sqlite3 "${DB_FILE}" " # vuln-code-snippet vuln-line sqlInjectionGetPreviousVersion
        SELECT version
        FROM deployments
        WHERE environment = '${environment}' AND status = 'completed'
        ORDER BY id DESC
        LIMIT 1 OFFSET ${offset}
    ")

    echo "${result}"
}
# vuln-code-snippet end sqlInjectionGetPreviousVersion

get_deployment_history() {
    local environment="$1"
    local limit="${2:-10}"

    db_query "
        SELECT id, version, status, started_at, deployed_by
        FROM deployments
        WHERE environment = '${environment}'
        ORDER BY id DESC
        LIMIT ${limit}
    "
}

# ============================================================================
# Deploy Locks
# ============================================================================
acquire_deploy_lock() {
    local environment="$1"
    local lock_holder="${USER:-unknown}@$(hostname)"
    local lock_duration="${2:-3600}"
    local expires_at
    expires_at=$(date -d "+${lock_duration} seconds" '+%Y-%m-%d %H:%M:%S' 2>/dev/null || date -v+${lock_duration}S '+%Y-%m-%d %H:%M:%S')

    # Check for existing lock
    local existing_lock
    existing_lock=$(sqlite3 "${DB_FILE}" "
        SELECT locked_by, expires_at
        FROM deploy_locks
        WHERE environment = '${environment}'
          AND (expires_at IS NULL OR expires_at > datetime('now'))
    ")

    if [[ -n "${existing_lock}" ]]; then
        log_warn "Lock already held: ${existing_lock}"
        return 1
    fi

    # Clear expired locks and acquire new one
    db_execute "
        DELETE FROM deploy_locks
        WHERE environment = '${environment}'
          AND expires_at <= datetime('now');

        INSERT OR REPLACE INTO deploy_locks (environment, locked_by, expires_at)
        VALUES ('${environment}', '${lock_holder}', '${expires_at}')
    "

    log_info "Acquired deploy lock for ${environment}"
    return 0
}

release_deploy_lock() {
    local environment="$1"

    db_execute "
        DELETE FROM deploy_locks
        WHERE environment = '${environment}'
    "

    log_info "Released deploy lock for ${environment}"
}

check_deploy_lock() {
    local environment="$1"

    local lock_info
    lock_info=$(sqlite3 "${DB_FILE}" "
        SELECT locked_by, expires_at
        FROM deploy_locks
        WHERE environment = '${environment}'
          AND (expires_at IS NULL OR expires_at > datetime('now'))
    ")

    if [[ -n "${lock_info}" ]]; then
        echo "${lock_info}"
        return 0
    else
        return 1
    fi
}

# ============================================================================
# Migration Tracking
# ============================================================================
record_migration() {
    local migration_id="$1"
    local checksum="${2:-}"

    db_execute "
        INSERT OR IGNORE INTO migrations (migration_id, checksum)
        VALUES ('${migration_id}', '${checksum}')
    "
}

is_migration_applied() {
    local migration_id="$1"

    local result
    result=$(sqlite3 "${DB_FILE}" "
        SELECT COUNT(*)
        FROM migrations
        WHERE migration_id = '${migration_id}'
    ")

    [[ "${result}" -gt 0 ]]
}

get_applied_migrations() {
    sqlite3 "${DB_FILE}" "
        SELECT migration_id, applied_at
        FROM migrations
        ORDER BY id
    "
}

# ============================================================================
# Health Check Records
# ============================================================================
# vuln-code-snippet start sqlInjectionRecordHealthCheck
record_health_check() {
    local service="$1"
    local status="$2"
    local response_time="${3:-}"
    local details="${4:-}"

    db_execute "
        INSERT INTO health_checks (service, status, response_time_ms, details)
        VALUES ('${service}', '${status}', ${response_time:-NULL}, '${details}') # vuln-code-snippet vuln-line sqlInjectionRecordHealthCheck
    "
}
# vuln-code-snippet end sqlInjectionRecordHealthCheck

get_health_history() {
    local service="$1"
    local limit="${2:-100}"

    db_query "
        SELECT status, response_time_ms, checked_at, details
        FROM health_checks
        WHERE service = '${service}'
        ORDER BY id DESC
        LIMIT ${limit}
    "
}

# ============================================================================
# Configuration History
# ============================================================================
# vuln-code-snippet start sqlInjectionRecordConfigChange
record_config_change() {
    local key="$1"
    local old_value="$2"
    local new_value="$3"
    local changed_by="${USER:-unknown}"

    db_execute "
        INSERT INTO config_history (key, old_value, new_value, changed_by)
        VALUES ('${key}', '${old_value}', '${new_value}', '${changed_by}') # vuln-code-snippet vuln-line sqlInjectionRecordConfigChange
    "
}
# vuln-code-snippet end sqlInjectionRecordConfigChange

get_config_history() {
    local key="$1"
    local limit="${2:-20}"

    db_query "
        SELECT old_value, new_value, changed_at, changed_by
        FROM config_history
        WHERE key = '${key}'
        ORDER BY id DESC
        LIMIT ${limit}
    "
}

# ============================================================================
# Database Maintenance
# ============================================================================
vacuum_database() {
    log_info "Running database vacuum"
    sqlite3 "${DB_FILE}" "VACUUM"
}

backup_database() {
    local backup_path="${1:-${DB_FILE}.backup}"

    log_info "Backing up database to ${backup_path}"
    sqlite3 "${DB_FILE}" ".backup '${backup_path}'"
}

# vuln-code-snippet start sqlInjectionCleanupDays
# TRIGGERS: rm with variable (for cleanup)
cleanup_old_records() {
    local days="${1:-30}"

    db_execute "
        DELETE FROM health_checks
        WHERE checked_at < datetime('now', '-${days} days') # vuln-code-snippet vuln-line sqlInjectionCleanupDays
    "

    db_execute "
        DELETE FROM config_history
        WHERE changed_at < datetime('now', '-${days} days')
    "

    log_info "Cleaned up records older than ${days} days"
}
# vuln-code-snippet end sqlInjectionCleanupDays
